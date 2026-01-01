/* FP:mod.rs-0001 */ use std::fmt::{self, Debug};
/* FP:mod.rs-0002 */ use std::hash::Hash;
/* FP:mod.rs-0003 */ use std::ops::RangeInclusive;
/* FP:mod.rs-0004 */ 
/* FP:mod.rs-0006 */ pub(crate) use tree::Tree;
/* FP:mod.rs-0007 */ 
/* FP:mod.rs-0009 */ pub(crate) use dfa::{Dfa, union};
/* FP:mod.rs-0010 */ 
/* FP:mod.rs-0011 */ #[derive(Debug)]
/* FP:mod.rs-0012 */ pub(crate) struct Uninhabited;
/* FP:mod.rs-0013 */ 
/* FP:mod.rs-0014 */ /// A range of byte values (including an uninit byte value).
/* FP:mod.rs-0015 */ #[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
/* FP:mod.rs-0016 */ pub(crate) struct Byte {
/* FP:mod.rs-0017 */     // An inclusive-exclusive range. We use this instead of `Range` because `Range: !Copy`.
/* FP:mod.rs-0018 */     //
/* FP:mod.rs-0019 */     // Uninit byte value is represented by 256.
/* FP:mod.rs-0020 */     pub(crate) start: u16,
/* FP:mod.rs-0021 */     pub(crate) end: u16,
/* FP:mod.rs-0022 */ }
/* FP:mod.rs-0023 */ 
/* FP:mod.rs-0024 */ impl Byte {
/* FP:mod.rs-0025 */     const UNINIT: u16 = 256;
/* FP:mod.rs-0026 */ 
/* FP:mod.rs-0027 */     #[inline]
/* FP:mod.rs-0028 */     fn new(range: RangeInclusive<u8>) -> Self {
/* FP:mod.rs-0029 */         let start: u16 = (*range.start()).into();
/* FP:mod.rs-0030 */         let end: u16 = (*range.end()).into();
/* FP:mod.rs-0031 */         Byte { start, end: end + 1 }
/* FP:mod.rs-0032 */     }
/* FP:mod.rs-0033 */ 
/* FP:mod.rs-0034 */     #[inline]
/* FP:mod.rs-0035 */     fn from_val(val: u8) -> Self {
/* FP:mod.rs-0036 */         let val: u16 = val.into();
/* FP:mod.rs-0037 */         Byte { start: val, end: val + 1 }
/* FP:mod.rs-0038 */     }
/* FP:mod.rs-0039 */ 
/* FP:mod.rs-0040 */     #[inline]
/* FP:mod.rs-0041 */     fn uninit() -> Byte {
/* FP:mod.rs-0042 */         Byte { start: 0, end: Self::UNINIT + 1 }
/* FP:mod.rs-0043 */     }
/* FP:mod.rs-0044 */ 
/* FP:mod.rs-0045 */     #[inline]
/* FP:mod.rs-0046 */     fn is_empty(&self) -> bool {
/* FP:mod.rs-0047 */         self.start == self.end
/* FP:mod.rs-0048 */     }
/* FP:mod.rs-0049 */ 
/* FP:mod.rs-0050 */     #[inline]
/* FP:mod.rs-0051 */     fn contains_uninit(&self) -> bool {
/* FP:mod.rs-0052 */         self.start <= Self::UNINIT && Self::UNINIT < self.end
/* FP:mod.rs-0053 */     }
/* FP:mod.rs-0054 */ }
/* FP:mod.rs-0055 */ 
/* FP:mod.rs-0056 */ impl fmt::Debug for Byte {
/* FP:mod.rs-0057 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:mod.rs-0058 */         if self.start == Self::UNINIT && self.end == Self::UNINIT + 1 {
/* FP:mod.rs-0059 */             write!(f, "uninit")
/* FP:mod.rs-0060 */         } else if self.start <= Self::UNINIT && self.end == Self::UNINIT + 1 {
/* FP:mod.rs-0061 */             write!(f, "{}..{}|uninit", self.start, self.end - 1)
/* FP:mod.rs-0062 */         } else {
/* FP:mod.rs-0063 */             write!(f, "{}..{}", self.start, self.end)
/* FP:mod.rs-0064 */         }
/* FP:mod.rs-0065 */     }
/* FP:mod.rs-0066 */ }
/* FP:mod.rs-0067 */ 
/* FP:mod.rs-0068 */ impl From<RangeInclusive<u8>> for Byte {
/* FP:mod.rs-0069 */     fn from(src: RangeInclusive<u8>) -> Self {
/* FP:mod.rs-0070 */         Self::new(src)
/* FP:mod.rs-0071 */     }
/* FP:mod.rs-0072 */ }
/* FP:mod.rs-0073 */ 
/* FP:mod.rs-0074 */ impl From<u8> for Byte {
/* FP:mod.rs-0075 */     #[inline]
/* FP:mod.rs-0076 */     fn from(src: u8) -> Self {
/* FP:mod.rs-0077 */         Self::from_val(src)
/* FP:mod.rs-0078 */     }
/* FP:mod.rs-0079 */ }
/* FP:mod.rs-0080 */ 
/* FP:mod.rs-0081 */ /// A reference, i.e., `&'region T` or `&'region mut T`.
/* FP:mod.rs-0082 */ #[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
/* FP:mod.rs-0083 */ pub(crate) struct Reference<R, T>
/* FP:mod.rs-0084 */ where
/* FP:mod.rs-0085 */     R: Region,
/* FP:mod.rs-0086 */     T: Type,
/* FP:mod.rs-0087 */ {
/* FP:mod.rs-0088 */     pub(crate) region: R,
/* FP:mod.rs-0089 */     pub(crate) is_mut: bool,
/* FP:mod.rs-0090 */     pub(crate) referent: T,
/* FP:mod.rs-0091 */     pub(crate) referent_size: usize,
/* FP:mod.rs-0092 */     pub(crate) referent_align: usize,
/* FP:mod.rs-0093 */ }
/* FP:mod.rs-0094 */ 
/* FP:mod.rs-0095 */ impl<R, T> fmt::Display for Reference<R, T>
/* FP:mod.rs-0096 */ where
/* FP:mod.rs-0097 */     R: Region,
/* FP:mod.rs-0098 */     T: Type,
/* FP:mod.rs-0099 */ {
/* FP:mod.rs-0100 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:mod.rs-0101 */         f.write_str("&")?;
/* FP:mod.rs-0102 */         if self.is_mut {
/* FP:mod.rs-0103 */             f.write_str("mut ")?;
/* FP:mod.rs-0104 */         }
/* FP:mod.rs-0105 */         self.referent.fmt(f)
/* FP:mod.rs-0106 */     }
/* FP:mod.rs-0107 */ }
/* FP:mod.rs-0108 */ 
/* FP:mod.rs-0109 */ pub(crate) trait Def: Debug + Hash + Eq + PartialEq + Copy + Clone {
/* FP:mod.rs-0110 */     fn has_safety_invariants(&self) -> bool;
/* FP:mod.rs-0111 */ }
/* FP:mod.rs-0112 */ 
/* FP:mod.rs-0113 */ pub(crate) trait Region: Debug + Hash + Eq + PartialEq + Copy + Clone {}
/* FP:mod.rs-0114 */ 
/* FP:mod.rs-0115 */ pub(crate) trait Type: Debug + Hash + Eq + PartialEq + Copy + Clone {}
/* FP:mod.rs-0116 */ 
/* FP:mod.rs-0117 */ impl Def for ! {
/* FP:mod.rs-0118 */     fn has_safety_invariants(&self) -> bool {
/* FP:mod.rs-0119 */         unreachable!()
/* FP:mod.rs-0120 */     }
/* FP:mod.rs-0121 */ }
/* FP:mod.rs-0122 */ 
/* FP:mod.rs-0123 */ impl Region for ! {}
/* FP:mod.rs-0124 */ 
/* FP:mod.rs-0125 */ impl Type for ! {}
/* FP:mod.rs-0126 */ 
/* FP:mod.rs-0127 */ #[cfg(test)]
/* FP:mod.rs-0128 */ impl Region for usize {}
/* FP:mod.rs-0129 */ 
/* FP:mod.rs-0130 */ #[cfg(test)]
/* FP:mod.rs-0131 */ impl Type for () {}
/* FP:mod.rs-0132 */ 
/* FP:mod.rs-0133 */ #[cfg(feature = "rustc")]
/* FP:mod.rs-0134 */ pub mod rustc {
/* FP:mod.rs-0135 */     use crate::rustc_abi::Layout;
/* FP:mod.rs-0136 */     use crate::rustc_complete::ty::layout::{HasTyCtxt, LayoutCx, LayoutError};
/* FP:mod.rs-0137 */     use crate::rustc_complete::ty::{self, Region, Ty};
/* FP:mod.rs-0138 */ 
/* FP:mod.rs-0139 */     /// A visibility node in the layout.
/* FP:mod.rs-0140 */     #[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
/* FP:mod.rs-0141 */     pub enum Def<'tcx> {
/* FP:mod.rs-0142 */         Adt(ty::AdtDef<'tcx>),
/* FP:mod.rs-0143 */         Variant(&'tcx ty::VariantDef),
/* FP:mod.rs-0144 */         Field(&'tcx ty::FieldDef),
/* FP:mod.rs-0145 */         Primitive,
/* FP:mod.rs-0146 */     }
/* FP:mod.rs-0147 */ 
/* FP:mod.rs-0148 */     impl<'tcx> super::Def for Def<'tcx> {
/* FP:mod.rs-0149 */         fn has_safety_invariants(&self) -> bool {
/* FP:mod.rs-0150 */             // Rust presently has no notion of 'unsafe fields', so for now we
/* FP:mod.rs-0151 */             // make the conservative assumption that everything besides
/* FP:mod.rs-0152 */             // primitive types carry safety invariants.
/* FP:mod.rs-0153 */             self != &Self::Primitive
/* FP:mod.rs-0154 */         }
/* FP:mod.rs-0155 */     }
/* FP:mod.rs-0156 */ 
/* FP:mod.rs-0157 */     impl<'tcx> super::Region for Region<'tcx> {}
/* FP:mod.rs-0158 */ 
/* FP:mod.rs-0159 */     impl<'tcx> super::Type for Ty<'tcx> {}
/* FP:mod.rs-0160 */ 
/* FP:mod.rs-0161 */     pub(crate) fn layout_of<'tcx>(
/* FP:mod.rs-0162 */         cx: LayoutCx<'tcx>,
/* FP:mod.rs-0163 */         ty: Ty<'tcx>,
/* FP:mod.rs-0164 */     ) -> Result<Layout<'tcx>, &'tcx LayoutError<'tcx>> {
/* FP:mod.rs-0165 */         use crate::rustc_complete::ty::layout::LayoutOf;
/* FP:mod.rs-0166 */         let ty = cx.tcx().erase_and_anonymize_regions(ty);
/* FP:mod.rs-0167 */         cx.layout_of(ty).map(|tl| tl.layout)
/* FP:mod.rs-0168 */     }
/* FP:mod.rs-0169 */ }
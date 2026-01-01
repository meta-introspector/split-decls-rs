/* FP:tree.rs-0001 */ use std::ops::{ControlFlow, RangeInclusive};
/* FP:tree.rs-0002 */ 
/* FP:tree.rs-0003 */ use super::{Byte, Def, Reference, Region, Type};
/* FP:tree.rs-0004 */ 
/* FP:tree.rs-0005 */ #[cfg(test)]
/* FP:tree.rs-0007 */ 
/* FP:tree.rs-0008 */ /// A tree-based representation of a type layout.
/* FP:tree.rs-0009 */ ///
/* FP:tree.rs-0010 */ /// Invariants:
/* FP:tree.rs-0011 */ /// 1. All paths through the layout have the same length (in bytes).
/* FP:tree.rs-0012 */ ///
/* FP:tree.rs-0013 */ /// Nice-to-haves:
/* FP:tree.rs-0014 */ /// 1. An `Alt` is never directly nested beneath another `Alt`.
/* FP:tree.rs-0015 */ /// 2. A `Seq` is never directly nested beneath another `Seq`.
/* FP:tree.rs-0016 */ /// 3. `Seq`s and `Alt`s with a single member do not exist.
/* FP:tree.rs-0017 */ #[derive(Clone, Debug, Hash, PartialEq, Eq)]
/* FP:tree.rs-0018 */ pub(crate) enum Tree<D, R, T>
/* FP:tree.rs-0019 */ where
/* FP:tree.rs-0020 */     D: Def,
/* FP:tree.rs-0021 */     R: Region,
/* FP:tree.rs-0022 */     T: Type,
/* FP:tree.rs-0023 */ {
/* FP:tree.rs-0024 */     /// A sequence of successive layouts.
/* FP:tree.rs-0025 */     Seq(Vec<Self>),
/* FP:tree.rs-0026 */     /// A choice between alternative layouts.
/* FP:tree.rs-0027 */     Alt(Vec<Self>),
/* FP:tree.rs-0028 */     /// A definition node.
/* FP:tree.rs-0029 */     Def(D),
/* FP:tree.rs-0030 */     /// A reference node.
/* FP:tree.rs-0031 */     Ref(Reference<R, T>),
/* FP:tree.rs-0032 */     /// A byte node.
/* FP:tree.rs-0033 */     Byte(Byte),
/* FP:tree.rs-0034 */ }
/* FP:tree.rs-0035 */ 
/* FP:tree.rs-0036 */ #[derive(Debug, Copy, Clone, Eq, PartialEq)]
/* FP:tree.rs-0037 */ pub(crate) enum Endian {
/* FP:tree.rs-0038 */     Little,
/* FP:tree.rs-0039 */     Big,
/* FP:tree.rs-0040 */ }
/* FP:tree.rs-0041 */ 
/* FP:tree.rs-0042 */ #[cfg(feature = "rustc")]
/* FP:tree.rs-0043 */ impl From<crate::rustc_abi::Endian> for Endian {
/* FP:tree.rs-0044 */     fn from(order: crate::rustc_abi::Endian) -> Endian {
/* FP:tree.rs-0045 */         match order {
/* FP:tree.rs-0046 */             crate::rustc_abi::Endian::Little => Endian::Little,
/* FP:tree.rs-0047 */             crate::rustc_abi::Endian::Big => Endian::Big,
/* FP:tree.rs-0048 */         }
/* FP:tree.rs-0049 */     }
/* FP:tree.rs-0050 */ }
/* FP:tree.rs-0051 */ 
/* FP:tree.rs-0052 */ impl<D, R, T> Tree<D, R, T>
/* FP:tree.rs-0053 */ where
/* FP:tree.rs-0054 */     D: Def,
/* FP:tree.rs-0055 */     R: Region,
/* FP:tree.rs-0056 */     T: Type,
/* FP:tree.rs-0057 */ {
/* FP:tree.rs-0058 */     /// A `Tree` consisting only of a definition node.
/* FP:tree.rs-0059 */     pub(crate) fn def(def: D) -> Self {
/* FP:tree.rs-0060 */         Self::Def(def)
/* FP:tree.rs-0061 */     }
/* FP:tree.rs-0062 */ 
/* FP:tree.rs-0063 */     /// A `Tree` representing an uninhabited type.
/* FP:tree.rs-0064 */     pub(crate) fn uninhabited() -> Self {
/* FP:tree.rs-0065 */         Self::Alt(vec![])
/* FP:tree.rs-0066 */     }
/* FP:tree.rs-0067 */ 
/* FP:tree.rs-0068 */     /// A `Tree` representing a zero-sized type.
/* FP:tree.rs-0069 */     pub(crate) fn unit() -> Self {
/* FP:tree.rs-0070 */         Self::Seq(Vec::new())
/* FP:tree.rs-0071 */     }
/* FP:tree.rs-0072 */ 
/* FP:tree.rs-0073 */     /// A `Tree` containing a single, uninitialized byte.
/* FP:tree.rs-0074 */     pub(crate) fn uninit() -> Self {
/* FP:tree.rs-0075 */         Self::Byte(Byte::uninit())
/* FP:tree.rs-0076 */     }
/* FP:tree.rs-0077 */ 
/* FP:tree.rs-0078 */     /// A `Tree` representing the layout of `bool`.
/* FP:tree.rs-0079 */     pub(crate) fn bool() -> Self {
/* FP:tree.rs-0080 */         Self::byte(0x00..=0x01)
/* FP:tree.rs-0081 */     }
/* FP:tree.rs-0082 */ 
/* FP:tree.rs-0083 */     /// A `Tree` whose layout matches that of a `u8`.
/* FP:tree.rs-0084 */     pub(crate) fn u8() -> Self {
/* FP:tree.rs-0085 */         Self::byte(0x00..=0xFF)
/* FP:tree.rs-0086 */     }
/* FP:tree.rs-0087 */ 
/* FP:tree.rs-0088 */     /// A `Tree` whose layout matches that of a `char`.
/* FP:tree.rs-0089 */     pub(crate) fn char(order: Endian) -> Self {
/* FP:tree.rs-0090 */         // `char`s can be in the following ranges:
/* FP:tree.rs-0091 */         // - [0, 0xD7FF]
/* FP:tree.rs-0092 */         // - [0xE000, 10FFFF]
/* FP:tree.rs-0093 */         //
/* FP:tree.rs-0094 */         // All other `char` values are illegal. We can thus represent a `char`
/* FP:tree.rs-0095 */         // as a union of three possible layouts:
/* FP:tree.rs-0096 */         // - 00 00 [00, D7] XX
/* FP:tree.rs-0097 */         // - 00 00 [E0, FF] XX
/* FP:tree.rs-0098 */         // - 00 [01, 10] XX XX
/* FP:tree.rs-0099 */ 
/* FP:tree.rs-0100 */         const _0: RangeInclusive<u8> = 0..=0;
/* FP:tree.rs-0101 */         const BYTE: RangeInclusive<u8> = 0x00..=0xFF;
/* FP:tree.rs-0102 */         let x = Self::from_big_endian(order, [_0, _0, 0x00..=0xD7, BYTE]);
/* FP:tree.rs-0103 */         let y = Self::from_big_endian(order, [_0, _0, 0xE0..=0xFF, BYTE]);
/* FP:tree.rs-0104 */         let z = Self::from_big_endian(order, [_0, 0x01..=0x10, BYTE, BYTE]);
/* FP:tree.rs-0105 */         Self::alt([x, y, z])
/* FP:tree.rs-0106 */     }
/* FP:tree.rs-0107 */ 
/* FP:tree.rs-0108 */     /// A `Tree` whose layout matches `std::num::NonZeroXxx`.
/* FP:tree.rs-0109 */     #[allow(dead_code)]
/* FP:tree.rs-0110 */     pub(crate) fn nonzero(width_in_bytes: u64) -> Self {
/* FP:tree.rs-0111 */         const BYTE: RangeInclusive<u8> = 0x00..=0xFF;
/* FP:tree.rs-0112 */         const NONZERO: RangeInclusive<u8> = 0x01..=0xFF;
/* FP:tree.rs-0113 */ 
/* FP:tree.rs-0114 */         (0..width_in_bytes)
/* FP:tree.rs-0115 */             .map(|nz_idx| {
/* FP:tree.rs-0116 */                 (0..width_in_bytes)
/* FP:tree.rs-0117 */                     .map(|pos| Self::byte(if pos == nz_idx { NONZERO } else { BYTE }))
/* FP:tree.rs-0118 */                     .fold(Self::unit(), Self::then)
/* FP:tree.rs-0119 */             })
/* FP:tree.rs-0120 */             .fold(Self::uninhabited(), Self::or)
/* FP:tree.rs-0121 */     }
/* FP:tree.rs-0122 */ 
/* FP:tree.rs-0123 */     pub(crate) fn bytes<const N: usize, B: Into<Byte>>(bytes: [B; N]) -> Self {
/* FP:tree.rs-0124 */         Self::seq(bytes.map(B::into).map(Self::Byte))
/* FP:tree.rs-0125 */     }
/* FP:tree.rs-0126 */ 
/* FP:tree.rs-0127 */     pub(crate) fn byte(byte: impl Into<Byte>) -> Self {
/* FP:tree.rs-0128 */         Self::Byte(byte.into())
/* FP:tree.rs-0129 */     }
/* FP:tree.rs-0130 */ 
/* FP:tree.rs-0131 */     /// A `Tree` whose layout is a number of the given width.
/* FP:tree.rs-0132 */     pub(crate) fn number(width_in_bytes: u64) -> Self {
/* FP:tree.rs-0133 */         Self::Seq(vec![Self::u8(); width_in_bytes.try_into().unwrap()])
/* FP:tree.rs-0134 */     }
/* FP:tree.rs-0135 */ 
/* FP:tree.rs-0136 */     /// A `Tree` whose layout is entirely padding of the given width.
/* FP:tree.rs-0137 */     pub(crate) fn padding(width_in_bytes: usize) -> Self {
/* FP:tree.rs-0138 */         Self::Seq(vec![Self::uninit(); width_in_bytes])
/* FP:tree.rs-0139 */     }
/* FP:tree.rs-0140 */ 
/* FP:tree.rs-0141 */     /// Remove all `Def` nodes, and all branches of the layout for which `f`
/* FP:tree.rs-0142 */     /// produces `true`.
/* FP:tree.rs-0143 */     pub(crate) fn prune<F>(self, f: &F) -> Tree<!, R, T>
/* FP:tree.rs-0144 */     where
/* FP:tree.rs-0145 */         F: Fn(D) -> bool,
/* FP:tree.rs-0146 */     {
/* FP:tree.rs-0147 */         match self {
/* FP:tree.rs-0148 */             Self::Seq(elts) => match elts.into_iter().map(|elt| elt.prune(f)).try_fold(
/* FP:tree.rs-0149 */                 Tree::unit(),
/* FP:tree.rs-0150 */                 |elts, elt| {
/* FP:tree.rs-0151 */                     if elt == Tree::uninhabited() {
/* FP:tree.rs-0152 */                         ControlFlow::Break(Tree::uninhabited())
/* FP:tree.rs-0153 */                     } else {
/* FP:tree.rs-0154 */                         ControlFlow::Continue(elts.then(elt))
/* FP:tree.rs-0155 */                     }
/* FP:tree.rs-0156 */                 },
/* FP:tree.rs-0157 */             ) {
/* FP:tree.rs-0158 */                 ControlFlow::Break(node) | ControlFlow::Continue(node) => node,
/* FP:tree.rs-0159 */             },
/* FP:tree.rs-0160 */             Self::Alt(alts) => alts
/* FP:tree.rs-0161 */                 .into_iter()
/* FP:tree.rs-0162 */                 .map(|alt| alt.prune(f))
/* FP:tree.rs-0163 */                 .fold(Tree::uninhabited(), |alts, alt| alts.or(alt)),
/* FP:tree.rs-0164 */             Self::Byte(b) => Tree::Byte(b),
/* FP:tree.rs-0165 */             Self::Ref(r) => Tree::Ref(r),
/* FP:tree.rs-0166 */             Self::Def(d) => {
/* FP:tree.rs-0167 */                 if f(d) {
/* FP:tree.rs-0168 */                     Tree::uninhabited()
/* FP:tree.rs-0169 */                 } else {
/* FP:tree.rs-0170 */                     Tree::unit()
/* FP:tree.rs-0171 */                 }
/* FP:tree.rs-0172 */             }
/* FP:tree.rs-0173 */         }
/* FP:tree.rs-0174 */     }
/* FP:tree.rs-0175 */ 
/* FP:tree.rs-0176 */     /// Produces `true` if `Tree` is an inhabited type; otherwise false.
/* FP:tree.rs-0177 */     pub(crate) fn is_inhabited(&self) -> bool {
/* FP:tree.rs-0178 */         match self {
/* FP:tree.rs-0179 */             Self::Seq(elts) => elts.into_iter().all(|elt| elt.is_inhabited()),
/* FP:tree.rs-0180 */             Self::Alt(alts) => alts.into_iter().any(|alt| alt.is_inhabited()),
/* FP:tree.rs-0181 */             Self::Byte(..) | Self::Ref(..) | Self::Def(..) => true,
/* FP:tree.rs-0182 */         }
/* FP:tree.rs-0183 */     }
/* FP:tree.rs-0184 */ 
/* FP:tree.rs-0185 */     /// Produces a `Tree` which represents a sequence of bytes stored in
/* FP:tree.rs-0186 */     /// `order`.
/* FP:tree.rs-0187 */     ///
/* FP:tree.rs-0188 */     /// `bytes` is taken to be in big-endian byte order, and its order will be
/* FP:tree.rs-0189 */     /// swapped if `order == Endian::Little`.
/* FP:tree.rs-0190 */     pub(crate) fn from_big_endian<const N: usize, B: Into<Byte>>(
/* FP:tree.rs-0191 */         order: Endian,
/* FP:tree.rs-0192 */         mut bytes: [B; N],
/* FP:tree.rs-0193 */     ) -> Self {
/* FP:tree.rs-0194 */         if order == Endian::Little {
/* FP:tree.rs-0195 */             (&mut bytes[..]).reverse();
/* FP:tree.rs-0196 */         }
/* FP:tree.rs-0197 */ 
/* FP:tree.rs-0198 */         Self::bytes(bytes)
/* FP:tree.rs-0199 */     }
/* FP:tree.rs-0200 */ 
/* FP:tree.rs-0201 */     /// Produces a `Tree` where each of the trees in `trees` are sequenced one
/* FP:tree.rs-0202 */     /// after another.
/* FP:tree.rs-0203 */     pub(crate) fn seq<const N: usize>(trees: [Tree<D, R, T>; N]) -> Self {
/* FP:tree.rs-0204 */         trees.into_iter().fold(Tree::unit(), Self::then)
/* FP:tree.rs-0205 */     }
/* FP:tree.rs-0206 */ 
/* FP:tree.rs-0207 */     /// Produces a `Tree` where each of the trees in `trees` are accepted as
/* FP:tree.rs-0208 */     /// alternative layouts.
/* FP:tree.rs-0209 */     pub(crate) fn alt<const N: usize>(trees: [Tree<D, R, T>; N]) -> Self {
/* FP:tree.rs-0210 */         trees.into_iter().fold(Tree::uninhabited(), Self::or)
/* FP:tree.rs-0211 */     }
/* FP:tree.rs-0212 */ 
/* FP:tree.rs-0213 */     /// Produces a new `Tree` where `other` is sequenced after `self`.
/* FP:tree.rs-0214 */     pub(crate) fn then(self, other: Self) -> Self {
/* FP:tree.rs-0215 */         match (self, other) {
/* FP:tree.rs-0216 */             (Self::Seq(elts), other) | (other, Self::Seq(elts)) if elts.len() == 0 => other,
/* FP:tree.rs-0217 */             (Self::Seq(mut lhs), Self::Seq(mut rhs)) => {
/* FP:tree.rs-0218 */                 lhs.append(&mut rhs);
/* FP:tree.rs-0219 */                 Self::Seq(lhs)
/* FP:tree.rs-0220 */             }
/* FP:tree.rs-0221 */             (Self::Seq(mut lhs), rhs) => {
/* FP:tree.rs-0222 */                 lhs.push(rhs);
/* FP:tree.rs-0223 */                 Self::Seq(lhs)
/* FP:tree.rs-0224 */             }
/* FP:tree.rs-0225 */             (lhs, Self::Seq(mut rhs)) => {
/* FP:tree.rs-0226 */                 rhs.insert(0, lhs);
/* FP:tree.rs-0227 */                 Self::Seq(rhs)
/* FP:tree.rs-0228 */             }
/* FP:tree.rs-0229 */             (lhs, rhs) => Self::Seq(vec![lhs, rhs]),
/* FP:tree.rs-0230 */         }
/* FP:tree.rs-0231 */     }
/* FP:tree.rs-0232 */ 
/* FP:tree.rs-0233 */     /// Produces a new `Tree` accepting either `self` or `other` as alternative layouts.
/* FP:tree.rs-0234 */     pub(crate) fn or(self, other: Self) -> Self {
/* FP:tree.rs-0235 */         match (self, other) {
/* FP:tree.rs-0236 */             (Self::Alt(alts), other) | (other, Self::Alt(alts)) if alts.len() == 0 => other,
/* FP:tree.rs-0237 */             (Self::Alt(mut lhs), Self::Alt(rhs)) => {
/* FP:tree.rs-0238 */                 lhs.extend(rhs);
/* FP:tree.rs-0239 */                 Self::Alt(lhs)
/* FP:tree.rs-0240 */             }
/* FP:tree.rs-0241 */             (Self::Alt(mut alts), alt) | (alt, Self::Alt(mut alts)) => {
/* FP:tree.rs-0242 */                 alts.push(alt);
/* FP:tree.rs-0243 */                 Self::Alt(alts)
/* FP:tree.rs-0244 */             }
/* FP:tree.rs-0245 */             (lhs, rhs) => Self::Alt(vec![lhs, rhs]),
/* FP:tree.rs-0246 */         }
/* FP:tree.rs-0247 */     }
/* FP:tree.rs-0248 */ }
/* FP:tree.rs-0249 */ 
/* FP:tree.rs-0250 */ #[cfg(feature = "rustc")]
/* FP:tree.rs-0251 */ pub(crate) mod rustc {
/* FP:tree.rs-0252 */     use crate::rustc_abi::{
/* FP:tree.rs-0253 */         FieldIdx, FieldsShape, Layout, Size, TagEncoding, TyAndLayout, VariantIdx, Variants,
/* FP:tree.rs-0254 */     };
/* FP:tree.rs-0255 */     use crate::rustc_complete::ty::layout::{HasTyCtxt, LayoutCx, LayoutError};
/* FP:tree.rs-0256 */     use crate::rustc_complete::ty::{
/* FP:tree.rs-0257 */         self, AdtDef, AdtKind, List, Region, ScalarInt, Ty, TyCtxt, TypeVisitableExt,
/* FP:tree.rs-0258 */     };
/* FP:tree.rs-0259 */     use crate::rustc_complete::ErrorGuaranteed;
/* FP:tree.rs-0260 */ 
/* FP:tree.rs-0261 */     use super::Tree;
/* FP:tree.rs-0262 */     use crate::layout::Reference;
/* FP:tree.rs-0263 */     use crate::layout::rustc::{Def, layout_of};
/* FP:tree.rs-0264 */ 
/* FP:tree.rs-0265 */     #[derive(Debug, Copy, Clone)]
/* FP:tree.rs-0266 */     pub(crate) enum Err {
/* FP:tree.rs-0267 */         /// The layout of the type is not yet supported.
/* FP:tree.rs-0268 */         NotYetSupported,
/* FP:tree.rs-0269 */         /// This error will be surfaced elsewhere by rustc, so don't surface it.
/* FP:tree.rs-0270 */         UnknownLayout,
/* FP:tree.rs-0271 */         /// Overflow size
/* FP:tree.rs-0272 */         SizeOverflow,
/* FP:tree.rs-0273 */         TypeError(ErrorGuaranteed),
/* FP:tree.rs-0274 */     }
/* FP:tree.rs-0275 */ 
/* FP:tree.rs-0276 */     impl<'tcx> From<&LayoutError<'tcx>> for Err {
/* FP:tree.rs-0277 */         fn from(err: &LayoutError<'tcx>) -> Self {
/* FP:tree.rs-0278 */             match err {
/* FP:tree.rs-0279 */                 LayoutError::Unknown(..)
/* FP:tree.rs-0280 */                 | LayoutError::ReferencesError(..)
/* FP:tree.rs-0281 */                 | LayoutError::TooGeneric(..)
/* FP:tree.rs-0282 */                 | LayoutError::NormalizationFailure(..) => Self::UnknownLayout,
/* FP:tree.rs-0283 */                 LayoutError::SizeOverflow(..) => Self::SizeOverflow,
/* FP:tree.rs-0284 */                 LayoutError::Cycle(err) => Self::TypeError(*err),
/* FP:tree.rs-0285 */             }
/* FP:tree.rs-0286 */         }
/* FP:tree.rs-0287 */     }
/* FP:tree.rs-0288 */ 
/* FP:tree.rs-0289 */     impl<'tcx> Tree<Def<'tcx>, Region<'tcx>, Ty<'tcx>> {
/* FP:tree.rs-0290 */         pub(crate) fn from_ty(ty: Ty<'tcx>, cx: LayoutCx<'tcx>) -> Result<Self, Err> {
/* FP:tree.rs-0291 */             use crate::rustc_abi::HasDataLayout;
/* FP:tree.rs-0292 */             let layout = layout_of(cx, ty)?;
/* FP:tree.rs-0293 */ 
/* FP:tree.rs-0294 */             if let Err(e) = ty.error_reported() {
/* FP:tree.rs-0295 */                 return Err(Err::TypeError(e));
/* FP:tree.rs-0296 */             }
/* FP:tree.rs-0297 */ 
/* FP:tree.rs-0298 */             let target = cx.data_layout();
/* FP:tree.rs-0299 */             let pointer_size = target.pointer_size();
/* FP:tree.rs-0300 */ 
/* FP:tree.rs-0301 */             match ty.kind() {
/* FP:tree.rs-0302 */                 ty::Bool => Ok(Self::bool()),
/* FP:tree.rs-0303 */ 
/* FP:tree.rs-0304 */                 ty::Float(nty) => {
/* FP:tree.rs-0305 */                     let width = nty.bit_width() / 8;
/* FP:tree.rs-0306 */                     Ok(Self::number(width.try_into().unwrap()))
/* FP:tree.rs-0307 */                 }
/* FP:tree.rs-0308 */ 
/* FP:tree.rs-0309 */                 ty::Int(nty) => {
/* FP:tree.rs-0310 */                     let width = nty.normalize(pointer_size.bits() as _).bit_width().unwrap() / 8;
/* FP:tree.rs-0311 */                     Ok(Self::number(width.try_into().unwrap()))
/* FP:tree.rs-0312 */                 }
/* FP:tree.rs-0313 */ 
/* FP:tree.rs-0314 */                 ty::Uint(nty) => {
/* FP:tree.rs-0315 */                     let width = nty.normalize(pointer_size.bits() as _).bit_width().unwrap() / 8;
/* FP:tree.rs-0316 */                     Ok(Self::number(width.try_into().unwrap()))
/* FP:tree.rs-0317 */                 }
/* FP:tree.rs-0318 */ 
/* FP:tree.rs-0319 */                 ty::Tuple(members) => Self::from_tuple((ty, layout), members, cx),
/* FP:tree.rs-0320 */ 
/* FP:tree.rs-0321 */                 ty::Array(inner_ty, _len) => {
/* FP:tree.rs-0322 */                     let FieldsShape::Array { stride, count } = &layout.fields else {
/* FP:tree.rs-0323 */                         return Err(Err::NotYetSupported);
/* FP:tree.rs-0324 */                     };
/* FP:tree.rs-0325 */                     let inner_layout = layout_of(cx, *inner_ty)?;
/* FP:tree.rs-0326 */                     assert_eq!(*stride, inner_layout.size);
/* FP:tree.rs-0327 */                     let elt = Tree::from_ty(*inner_ty, cx)?;
/* FP:tree.rs-0328 */                     Ok(std::iter::repeat(elt)
/* FP:tree.rs-0329 */                         .take(*count as usize)
/* FP:tree.rs-0330 */                         .fold(Tree::unit(), |tree, elt| tree.then(elt)))
/* FP:tree.rs-0331 */                 }
/* FP:tree.rs-0332 */ 
/* FP:tree.rs-0333 */                 ty::Adt(adt_def, _args_ref) if !ty.is_box() => {
/* FP:tree.rs-0334 */                     let (lo, hi) = cx.tcx().layout_scalar_valid_range(adt_def.did());
/* FP:tree.rs-0335 */ 
/* FP:tree.rs-0336 */                     use core::ops::Bound::*;
/* FP:tree.rs-0337 */                     let is_transparent = adt_def.repr().transparent();
/* FP:tree.rs-0338 */                     match (adt_def.adt_kind(), lo, hi) {
/* FP:tree.rs-0339 */                         (AdtKind::Struct, Unbounded, Unbounded) => {
/* FP:tree.rs-0340 */                             Self::from_struct((ty, layout), *adt_def, cx)
/* FP:tree.rs-0341 */                         }
/* FP:tree.rs-0342 */                         (AdtKind::Struct, Included(1), Included(_hi)) if is_transparent => {
/* FP:tree.rs-0343 */                             // FIXME(@joshlf): Support `NonZero` types:
/* FP:tree.rs-0344 */                             // - Check to make sure that the first field is
/* FP:tree.rs-0345 */                             //   numerical
/* FP:tree.rs-0346 */                             // - Check to make sure that the upper bound is the
/* FP:tree.rs-0347 */                             //   maximum value for the field's type
/* FP:tree.rs-0348 */                             // - Construct `Self::nonzero`
/* FP:tree.rs-0349 */                             Err(Err::NotYetSupported)
/* FP:tree.rs-0350 */                         }
/* FP:tree.rs-0351 */                         (AdtKind::Enum, Unbounded, Unbounded) => {
/* FP:tree.rs-0352 */                             Self::from_enum((ty, layout), *adt_def, cx)
/* FP:tree.rs-0353 */                         }
/* FP:tree.rs-0354 */                         (AdtKind::Union, Unbounded, Unbounded) => {
/* FP:tree.rs-0355 */                             Self::from_union((ty, layout), *adt_def, cx)
/* FP:tree.rs-0356 */                         }
/* FP:tree.rs-0357 */                         _ => Err(Err::NotYetSupported),
/* FP:tree.rs-0358 */                     }
/* FP:tree.rs-0359 */                 }
/* FP:tree.rs-0360 */ 
/* FP:tree.rs-0361 */                 ty::Ref(region, ty, mutability) => {
/* FP:tree.rs-0362 */                     let layout = layout_of(cx, *ty)?;
/* FP:tree.rs-0363 */                     let referent_align = layout.align.abi.bytes_usize();
/* FP:tree.rs-0364 */                     let referent_size = layout.size.bytes_usize();
/* FP:tree.rs-0365 */ 
/* FP:tree.rs-0366 */                     Ok(Tree::Ref(Reference {
/* FP:tree.rs-0367 */                         region: *region,
/* FP:tree.rs-0368 */                         is_mut: mutability.is_mut(),
/* FP:tree.rs-0369 */                         referent: *ty,
/* FP:tree.rs-0370 */                         referent_align,
/* FP:tree.rs-0371 */                         referent_size,
/* FP:tree.rs-0372 */                     }))
/* FP:tree.rs-0373 */                 }
/* FP:tree.rs-0374 */ 
/* FP:tree.rs-0375 */                 ty::Char => Ok(Self::char(cx.tcx().data_layout.endian.into())),
/* FP:tree.rs-0376 */ 
/* FP:tree.rs-0377 */                 _ => Err(Err::NotYetSupported),
/* FP:tree.rs-0378 */             }
/* FP:tree.rs-0379 */         }
/* FP:tree.rs-0380 */ 
/* FP:tree.rs-0381 */         /// Constructs a `Tree` from a tuple.
/* FP:tree.rs-0382 */         fn from_tuple(
/* FP:tree.rs-0383 */             (ty, layout): (Ty<'tcx>, Layout<'tcx>),
/* FP:tree.rs-0384 */             members: &'tcx List<Ty<'tcx>>,
/* FP:tree.rs-0385 */             cx: LayoutCx<'tcx>,
/* FP:tree.rs-0386 */         ) -> Result<Self, Err> {
/* FP:tree.rs-0387 */             match &layout.fields {
/* FP:tree.rs-0388 */                 FieldsShape::Primitive => {
/* FP:tree.rs-0389 */                     assert_eq!(members.len(), 1);
/* FP:tree.rs-0390 */                     let inner_ty = members[0];
/* FP:tree.rs-0391 */                     Self::from_ty(inner_ty, cx)
/* FP:tree.rs-0392 */                 }
/* FP:tree.rs-0393 */                 FieldsShape::Arbitrary { offsets, .. } => {
/* FP:tree.rs-0394 */                     assert_eq!(offsets.len(), members.len());
/* FP:tree.rs-0395 */                     Self::from_variant(Def::Primitive, None, (ty, layout), layout.size, cx)
/* FP:tree.rs-0396 */                 }
/* FP:tree.rs-0397 */                 FieldsShape::Array { .. } | FieldsShape::Union(_) => Err(Err::NotYetSupported),
/* FP:tree.rs-0398 */             }
/* FP:tree.rs-0399 */         }
/* FP:tree.rs-0400 */ 
/* FP:tree.rs-0401 */         /// Constructs a `Tree` from a struct.
/* FP:tree.rs-0402 */         ///
/* FP:tree.rs-0403 */         /// # Panics
/* FP:tree.rs-0404 */         ///
/* FP:tree.rs-0405 */         /// Panics if `def` is not a struct definition.
/* FP:tree.rs-0406 */         fn from_struct(
/* FP:tree.rs-0407 */             (ty, layout): (Ty<'tcx>, Layout<'tcx>),
/* FP:tree.rs-0408 */             def: AdtDef<'tcx>,
/* FP:tree.rs-0409 */             cx: LayoutCx<'tcx>,
/* FP:tree.rs-0410 */         ) -> Result<Self, Err> {
/* FP:tree.rs-0411 */             assert!(def.is_struct());
/* FP:tree.rs-0412 */             let def = Def::Adt(def);
/* FP:tree.rs-0413 */             Self::from_variant(def, None, (ty, layout), layout.size, cx)
/* FP:tree.rs-0414 */         }
/* FP:tree.rs-0415 */ 
/* FP:tree.rs-0416 */         /// Constructs a `Tree` from an enum.
/* FP:tree.rs-0417 */         ///
/* FP:tree.rs-0418 */         /// # Panics
/* FP:tree.rs-0419 */         ///
/* FP:tree.rs-0420 */         /// Panics if `def` is not an enum definition.
/* FP:tree.rs-0421 */         fn from_enum(
/* FP:tree.rs-0422 */             (ty, layout): (Ty<'tcx>, Layout<'tcx>),
/* FP:tree.rs-0423 */             def: AdtDef<'tcx>,
/* FP:tree.rs-0424 */             cx: LayoutCx<'tcx>,
/* FP:tree.rs-0425 */         ) -> Result<Self, Err> {
/* FP:tree.rs-0426 */             assert!(def.is_enum());
/* FP:tree.rs-0427 */ 
/* FP:tree.rs-0428 */             // Computes the layout of a variant.
/* FP:tree.rs-0429 */             let layout_of_variant = |index, encoding: Option<_>| -> Result<Self, Err> {
/* FP:tree.rs-0430 */                 let variant_layout = ty_variant(cx, (ty, layout), index);
/* FP:tree.rs-0431 */                 if variant_layout.is_uninhabited() {
/* FP:tree.rs-0432 */                     return Ok(Self::uninhabited());
/* FP:tree.rs-0433 */                 }
/* FP:tree.rs-0434 */                 let tag = cx.tcx().tag_for_variant(
/* FP:tree.rs-0435 */                     cx.typing_env.as_query_input((cx.tcx().erase_and_anonymize_regions(ty), index)),
/* FP:tree.rs-0436 */                 );
/* FP:tree.rs-0437 */                 let variant_def = Def::Variant(def.variant(index));
/* FP:tree.rs-0438 */                 Self::from_variant(
/* FP:tree.rs-0439 */                     variant_def,
/* FP:tree.rs-0440 */                     tag.map(|tag| (tag, index, encoding.unwrap())),
/* FP:tree.rs-0441 */                     (ty, variant_layout),
/* FP:tree.rs-0442 */                     layout.size,
/* FP:tree.rs-0443 */                     cx,
/* FP:tree.rs-0444 */                 )
/* FP:tree.rs-0445 */             };
/* FP:tree.rs-0446 */ 
/* FP:tree.rs-0447 */             match layout.variants() {
/* FP:tree.rs-0448 */                 Variants::Empty => Ok(Self::uninhabited()),
/* FP:tree.rs-0449 */                 Variants::Single { index } => {
/* FP:tree.rs-0450 */                     // `Variants::Single` on enums with variants denotes that
/* FP:tree.rs-0451 */                     // the enum delegates its layout to the variant at `index`.
/* FP:tree.rs-0452 */                     layout_of_variant(*index, None)
/* FP:tree.rs-0453 */                 }
/* FP:tree.rs-0454 */                 Variants::Multiple { tag: _, tag_encoding, tag_field, .. } => {
/* FP:tree.rs-0455 */                     // `Variants::Multiple` denotes an enum with multiple
/* FP:tree.rs-0456 */                     // variants. The layout of such an enum is the disjunction
/* FP:tree.rs-0457 */                     // of the layouts of its tagged variants.
/* FP:tree.rs-0458 */ 
/* FP:tree.rs-0459 */                     // For enums (but not coroutines), the tag field is
/* FP:tree.rs-0460 */                     // currently always the first field of the layout.
/* FP:tree.rs-0461 */                     assert_eq!(*tag_field, FieldIdx::ZERO);
/* FP:tree.rs-0462 */ 
/* FP:tree.rs-0463 */                     let variants = def.discriminants(cx.tcx()).try_fold(
/* FP:tree.rs-0464 */                         Self::uninhabited(),
/* FP:tree.rs-0465 */                         |variants, (idx, _discriminant)| {
/* FP:tree.rs-0466 */                             let variant = layout_of_variant(idx, Some(tag_encoding.clone()))?;
/* FP:tree.rs-0467 */                             Result::<Self, Err>::Ok(variants.or(variant))
/* FP:tree.rs-0468 */                         },
/* FP:tree.rs-0469 */                     )?;
/* FP:tree.rs-0470 */ 
/* FP:tree.rs-0471 */                     Ok(Self::def(Def::Adt(def)).then(variants))
/* FP:tree.rs-0472 */                 }
/* FP:tree.rs-0473 */             }
/* FP:tree.rs-0474 */         }
/* FP:tree.rs-0475 */ 
/* FP:tree.rs-0476 */         /// Constructs a `Tree` from a 'variant-like' layout.
/* FP:tree.rs-0477 */         ///
/* FP:tree.rs-0478 */         /// A 'variant-like' layout includes those of structs and, of course,
/* FP:tree.rs-0479 */         /// enum variants. Pragmatically speaking, this method supports anything
/* FP:tree.rs-0480 */         /// with `FieldsShape::Arbitrary`.
/* FP:tree.rs-0481 */         ///
/* FP:tree.rs-0482 */         /// Note: This routine assumes that the optional `tag` is the first
/* FP:tree.rs-0483 */         /// field, and enum callers should check that `tag_field` is, in fact,
/* FP:tree.rs-0484 */         /// `0`.
/* FP:tree.rs-0485 */         fn from_variant(
/* FP:tree.rs-0486 */             def: Def<'tcx>,
/* FP:tree.rs-0487 */             tag: Option<(ScalarInt, VariantIdx, TagEncoding<VariantIdx>)>,
/* FP:tree.rs-0488 */             (ty, layout): (Ty<'tcx>, Layout<'tcx>),
/* FP:tree.rs-0489 */             total_size: Size,
/* FP:tree.rs-0490 */             cx: LayoutCx<'tcx>,
/* FP:tree.rs-0491 */         ) -> Result<Self, Err> {
/* FP:tree.rs-0492 */             // This constructor does not support non-`FieldsShape::Arbitrary`
/* FP:tree.rs-0493 */             // layouts.
/* FP:tree.rs-0494 */             let FieldsShape::Arbitrary { offsets, memory_index } = layout.fields() else {
/* FP:tree.rs-0495 */                 return Err(Err::NotYetSupported);
/* FP:tree.rs-0496 */             };
/* FP:tree.rs-0497 */ 
/* FP:tree.rs-0498 */             // When this function is invoked with enum variants,
/* FP:tree.rs-0499 */             // `ty_and_layout.size` does not encompass the entire size of the
/* FP:tree.rs-0500 */             // enum. We rely on `total_size` for this.
/* FP:tree.rs-0501 */             assert!(layout.size <= total_size);
/* FP:tree.rs-0502 */ 
/* FP:tree.rs-0503 */             let mut size = Size::ZERO;
/* FP:tree.rs-0504 */             let mut struct_tree = Self::def(def);
/* FP:tree.rs-0505 */ 
/* FP:tree.rs-0506 */             // If a `tag` is provided, place it at the start of the layout.
/* FP:tree.rs-0507 */             if let Some((tag, index, encoding)) = &tag {
/* FP:tree.rs-0508 */                 match encoding {
/* FP:tree.rs-0509 */                     TagEncoding::Direct => {
/* FP:tree.rs-0510 */                         size += tag.size();
/* FP:tree.rs-0511 */                     }
/* FP:tree.rs-0512 */                     TagEncoding::Niche { niche_variants, .. } => {
/* FP:tree.rs-0513 */                         if !niche_variants.contains(index) {
/* FP:tree.rs-0514 */                             size += tag.size();
/* FP:tree.rs-0515 */                         }
/* FP:tree.rs-0516 */                     }
/* FP:tree.rs-0517 */                 }
/* FP:tree.rs-0518 */                 struct_tree = struct_tree.then(Self::from_tag(*tag, cx.tcx()));
/* FP:tree.rs-0519 */             }
/* FP:tree.rs-0520 */ 
/* FP:tree.rs-0521 */             // Append the fields, in memory order, to the layout.
/* FP:tree.rs-0522 */             let inverse_memory_index = memory_index.invert_bijective_mapping();
/* FP:tree.rs-0523 */             for &field_idx in inverse_memory_index.iter() {
/* FP:tree.rs-0524 */                 // Add interfield padding.
/* FP:tree.rs-0525 */                 let padding_needed = offsets[field_idx] - size;
/* FP:tree.rs-0526 */                 let padding = Self::padding(padding_needed.bytes_usize());
/* FP:tree.rs-0527 */ 
/* FP:tree.rs-0528 */                 let field_ty = ty_field(cx, (ty, layout), field_idx);
/* FP:tree.rs-0529 */                 let field_layout = layout_of(cx, field_ty)?;
/* FP:tree.rs-0530 */                 let field_tree = Self::from_ty(field_ty, cx)?;
/* FP:tree.rs-0531 */ 
/* FP:tree.rs-0532 */                 struct_tree = struct_tree.then(padding).then(field_tree);
/* FP:tree.rs-0533 */ 
/* FP:tree.rs-0534 */                 size += padding_needed + field_layout.size;
/* FP:tree.rs-0535 */             }
/* FP:tree.rs-0536 */ 
/* FP:tree.rs-0537 */             // Add trailing padding.
/* FP:tree.rs-0538 */             let padding_needed = total_size - size;
/* FP:tree.rs-0539 */             let trailing_padding = Self::padding(padding_needed.bytes_usize());
/* FP:tree.rs-0540 */ 
/* FP:tree.rs-0541 */             Ok(struct_tree.then(trailing_padding))
/* FP:tree.rs-0542 */         }
/* FP:tree.rs-0543 */ 
/* FP:tree.rs-0544 */         /// Constructs a `Tree` representing the value of a enum tag.
/* FP:tree.rs-0545 */         fn from_tag(tag: ScalarInt, tcx: TyCtxt<'tcx>) -> Self {
/* FP:tree.rs-0546 */             use crate::rustc_abi::Endian;
/* FP:tree.rs-0547 */             let size = tag.size();
/* FP:tree.rs-0548 */             let bits = tag.to_bits(size);
/* FP:tree.rs-0549 */             let bytes: [u8; 16];
/* FP:tree.rs-0550 */             let bytes = match tcx.data_layout.endian {
/* FP:tree.rs-0551 */                 Endian::Little => {
/* FP:tree.rs-0552 */                     bytes = bits.to_le_bytes();
/* FP:tree.rs-0553 */                     &bytes[..size.bytes_usize()]
/* FP:tree.rs-0554 */                 }
/* FP:tree.rs-0555 */                 Endian::Big => {
/* FP:tree.rs-0556 */                     bytes = bits.to_be_bytes();
/* FP:tree.rs-0557 */                     &bytes[bytes.len() - size.bytes_usize()..]
/* FP:tree.rs-0558 */                 }
/* FP:tree.rs-0559 */             };
/* FP:tree.rs-0560 */             Self::Seq(bytes.iter().map(|&b| Self::byte(b)).collect())
/* FP:tree.rs-0561 */         }
/* FP:tree.rs-0562 */ 
/* FP:tree.rs-0563 */         /// Constructs a `Tree` from a union.
/* FP:tree.rs-0564 */         ///
/* FP:tree.rs-0565 */         /// # Panics
/* FP:tree.rs-0566 */         ///
/* FP:tree.rs-0567 */         /// Panics if `def` is not a union definition.
/* FP:tree.rs-0568 */         fn from_union(
/* FP:tree.rs-0569 */             (ty, layout): (Ty<'tcx>, Layout<'tcx>),
/* FP:tree.rs-0570 */             def: AdtDef<'tcx>,
/* FP:tree.rs-0571 */             cx: LayoutCx<'tcx>,
/* FP:tree.rs-0572 */         ) -> Result<Self, Err> {
/* FP:tree.rs-0573 */             assert!(def.is_union());
/* FP:tree.rs-0574 */ 
/* FP:tree.rs-0575 */             // This constructor does not support non-`FieldsShape::Union`
/* FP:tree.rs-0576 */             // layouts. Fields of this shape are all placed at offset 0.
/* FP:tree.rs-0577 */             let FieldsShape::Union(_fields) = layout.fields() else {
/* FP:tree.rs-0578 */                 return Err(Err::NotYetSupported);
/* FP:tree.rs-0579 */             };
/* FP:tree.rs-0580 */ 
/* FP:tree.rs-0581 */             let fields = &def.non_enum_variant().fields;
/* FP:tree.rs-0582 */             let fields = fields.iter_enumerated().try_fold(
/* FP:tree.rs-0583 */                 Self::uninhabited(),
/* FP:tree.rs-0584 */                 |fields, (idx, _field_def)| {
/* FP:tree.rs-0585 */                     let field_ty = ty_field(cx, (ty, layout), idx);
/* FP:tree.rs-0586 */                     let field_layout = layout_of(cx, field_ty)?;
/* FP:tree.rs-0587 */                     let field = Self::from_ty(field_ty, cx)?;
/* FP:tree.rs-0588 */                     let trailing_padding_needed = layout.size - field_layout.size;
/* FP:tree.rs-0589 */                     let trailing_padding = Self::padding(trailing_padding_needed.bytes_usize());
/* FP:tree.rs-0590 */                     let field_and_padding = field.then(trailing_padding);
/* FP:tree.rs-0591 */                     Result::<Self, Err>::Ok(fields.or(field_and_padding))
/* FP:tree.rs-0592 */                 },
/* FP:tree.rs-0593 */             )?;
/* FP:tree.rs-0594 */ 
/* FP:tree.rs-0595 */             Ok(Self::def(Def::Adt(def)).then(fields))
/* FP:tree.rs-0596 */         }
/* FP:tree.rs-0597 */     }
/* FP:tree.rs-0598 */ 
/* FP:tree.rs-0599 */     fn ty_field<'tcx>(
/* FP:tree.rs-0600 */         cx: LayoutCx<'tcx>,
/* FP:tree.rs-0601 */         (ty, layout): (Ty<'tcx>, Layout<'tcx>),
/* FP:tree.rs-0602 */         i: FieldIdx,
/* FP:tree.rs-0603 */     ) -> Ty<'tcx> {
/* FP:tree.rs-0604 */         // We cannot use `ty_and_layout_field` to retrieve the field type, since
/* FP:tree.rs-0605 */         // `ty_and_layout_field` erases regions in the returned type. We must
/* FP:tree.rs-0606 */         // not erase regions here, since we may need to ultimately emit outlives
/* FP:tree.rs-0607 */         // obligations as a consequence of the transmutability analysis.
/* FP:tree.rs-0608 */         match ty.kind() {
/* FP:tree.rs-0609 */             ty::Adt(def, args) => {
/* FP:tree.rs-0610 */                 match layout.variants {
/* FP:tree.rs-0611 */                     Variants::Single { index } => {
/* FP:tree.rs-0612 */                         let field = &def.variant(index).fields[i];
/* FP:tree.rs-0613 */                         field.ty(cx.tcx(), args)
/* FP:tree.rs-0614 */                     }
/* FP:tree.rs-0615 */                     Variants::Empty => panic!("there is no field in Variants::Empty types"),
/* FP:tree.rs-0616 */                     // Discriminant field for enums (where applicable).
/* FP:tree.rs-0617 */                     Variants::Multiple { tag, .. } => {
/* FP:tree.rs-0618 */                         assert_eq!(i.as_usize(), 0);
/* FP:tree.rs-0619 */                         ty::layout::PrimitiveExt::to_ty(&tag.primitive(), cx.tcx())
/* FP:tree.rs-0620 */                     }
/* FP:tree.rs-0621 */                 }
/* FP:tree.rs-0622 */             }
/* FP:tree.rs-0623 */             ty::Tuple(fields) => fields[i.as_usize()],
/* FP:tree.rs-0624 */             kind => unimplemented!(
/* FP:tree.rs-0625 */                 "only a subset of `Ty::ty_and_layout_field`'s functionality is implemented. implementation needed for {:?}",
/* FP:tree.rs-0626 */                 kind
/* FP:tree.rs-0627 */             ),
/* FP:tree.rs-0628 */         }
/* FP:tree.rs-0629 */     }
/* FP:tree.rs-0630 */ 
/* FP:tree.rs-0631 */     fn ty_variant<'tcx>(
/* FP:tree.rs-0632 */         cx: LayoutCx<'tcx>,
/* FP:tree.rs-0633 */         (ty, layout): (Ty<'tcx>, Layout<'tcx>),
/* FP:tree.rs-0634 */         i: VariantIdx,
/* FP:tree.rs-0635 */     ) -> Layout<'tcx> {
/* FP:tree.rs-0636 */         let ty = cx.tcx().erase_and_anonymize_regions(ty);
/* FP:tree.rs-0637 */         TyAndLayout { ty, layout }.for_variant(&cx, i).layout
/* FP:tree.rs-0638 */     }
/* FP:tree.rs-0639 */ }
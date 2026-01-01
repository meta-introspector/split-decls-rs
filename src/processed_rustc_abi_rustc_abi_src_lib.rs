/* FP:lib.rs-0001 */ // tidy-alphabetical-start
/* FP:lib.rs-0002 */ #[cfg_attr(feature = "nightly", allow(internal_features))]
/* FP:lib.rs-0003 */ #[cfg_attr(feature = "nightly", doc(rust_logo))]
/* FP:lib.rs-0004 */ #[cfg_attr(feature = "nightly", feature(assert_matches))]
/* FP:lib.rs-0005 */ #[cfg_attr(feature = "nightly", feature(rustc_attrs))]
/* FP:lib.rs-0006 */ #[cfg_attr(feature = "nightly", feature(rustdoc_internals))]
/* FP:lib.rs-0007 */ #[cfg_attr(feature = "nightly", feature(step_trait))]
/* FP:lib.rs-0008 */ // tidy-alphabetical-end
/* FP:lib.rs-0009 */ 
/* FP:lib.rs-0010 */ /* ABI handling for rustc
/* FP:lib.rs-0011 */ 
/* FP:lib.rs-0012 */ ## What is an "ABI"?
/* FP:lib.rs-0013 */ 
/* FP:lib.rs-0014 */ Literally, "application binary interface", which means it is everything about how code interacts,
/* FP:lib.rs-0015 */ at the machine level, with other code. This means it technically covers all of the following:
/* FP:lib.rs-0016 */ - object binary format for e.g. relocations or offset tables
/* FP:lib.rs-0017 */ - in-memory layout of types
/* FP:lib.rs-0018 */ - procedure calling conventions
/* FP:lib.rs-0019 */ 
/* FP:lib.rs-0020 */ When we discuss "ABI" in the context of rustc, we are probably discussing calling conventions.
/* FP:lib.rs-0021 */ To describe those `rustc_abi` also covers type layout, as it must for values passed on the stack.
/* FP:lib.rs-0022 */ Despite `rustc_abi` being about calling conventions, it is good to remember these usages exist.
/* FP:lib.rs-0023 */ You will encounter all of them and more if you study target-specific codegen enough!
/* FP:lib.rs-0024 */ Even in general conversation, when someone says "the Rust ABI is unstable", it may allude to
/* FP:lib.rs-0025 */ either or both of
/* FP:lib.rs-0026 */ - `repr(Rust)` types have a mostly-unspecified layout
/* FP:lib.rs-0027 */ - `extern "Rust" fn(A) -> R` has an unspecified calling convention
/* FP:lib.rs-0028 */ 
/* FP:lib.rs-0029 */ ## Crate Goal
/* FP:lib.rs-0030 */ 
/* FP:lib.rs-0031 */ ABI is a foundational concept, so the `rustc_abi` crate serves as an equally foundational crate.
/* FP:lib.rs-0032 */ It cannot carry all details relevant to an ABI: those permeate code generation and linkage.
/* FP:lib.rs-0033 */ Instead, `rustc_abi` is intended to provide the interface for reasoning about the binary interface.
/* FP:lib.rs-0034 */ It should contain traits and types that other crates then use in their implementation.
/* FP:lib.rs-0035 */ For example, a platform's `extern "C" fn` calling convention will be implemented in `rustc_target`
/* FP:lib.rs-0036 */ but `rustc_abi` contains the types for calculating layout and describing register-passing.
/* FP:lib.rs-0037 */ This makes it easier to describe things in the same way across targets, codegen backends, and
/* FP:lib.rs-0038 */ even other Rust compilers, such as rust-analyzer!
/* FP:lib.rs-0039 */ 
/* FP:lib.rs-0040 */ */
/* FP:lib.rs-0041 */ 
/* FP:lib.rs-0042 */ use std::fmt;
/* FP:lib.rs-0043 */ #[cfg(feature = "nightly")]
/* FP:lib.rs-0044 */ use std::iter::Step;
/* FP:lib.rs-0045 */ use std::num::{NonZeroUsize, ParseIntError};
/* FP:lib.rs-0046 */ use std::ops::{Add, AddAssign, Deref, Mul, RangeFull, RangeInclusive, Sub};
/* FP:lib.rs-0047 */ use std::str::FromStr;
/* FP:lib.rs-0048 */ 
/* FP:lib.rs-0049 */ use bitflags::bitflags;
/* FP:lib.rs-0050 */ #[cfg(feature = "nightly")]
/* FP:lib.rs-0051 */ use crate::rustc_data_structures::stable_hasher::StableOrd;
/* FP:lib.rs-0052 */ use rustc_hashes::Hash64;
/* FP:lib.rs-0053 */ use crate::rustc_index::{Idx, IndexSlice, IndexVec};
/* FP:lib.rs-0054 */ #[cfg(feature = "nightly")]
/* FP:lib.rs-0055 */ use rustc_macros::{Decodable_NoContext, Encodable_NoContext, HashStable_Generic};
/* FP:lib.rs-0056 */ 
/* FP:lib.rs-0061 */ #[cfg(test)]
/* FP:lib.rs-0063 */ 
/* FP:lib.rs-0064 */ pub use callconv::{Heterogeneous, HomogeneousAggregate, Reg, RegKind};
/* FP:lib.rs-0065 */ pub use canon_abi::{ArmCall, CanonAbi, InterruptKind, X86Call};
/* FP:lib.rs-0066 */ #[cfg(feature = "nightly")]
/* FP:lib.rs-0067 */ pub use extern_abi::CVariadicStatus;
/* FP:lib.rs-0068 */ pub use extern_abi::{ExternAbi, all_names};
/* FP:lib.rs-0069 */ #[cfg(feature = "nightly")]
/* FP:lib.rs-0070 */ pub use layout::{FIRST_VARIANT, FieldIdx, Layout, TyAbiInterface, TyAndLayout, VariantIdx};
/* FP:lib.rs-0071 */ pub use layout::{LayoutCalculator, LayoutCalculatorError};
/* FP:lib.rs-0072 */ 
/* FP:lib.rs-0073 */ /// Requirements for a `StableHashingContext` to be used in this crate.
/* FP:lib.rs-0074 */ /// This is a hack to allow using the `HashStable_Generic` derive macro
/* FP:lib.rs-0075 */ /// instead of implementing everything in `rustc_middle`.
/* FP:lib.rs-0076 */ #[cfg(feature = "nightly")]
/* FP:lib.rs-0077 */ pub trait HashStableContext {}
/* FP:lib.rs-0078 */ 
/* FP:lib.rs-0079 */ #[derive(Clone, Copy, PartialEq, Eq, Default)]
/* FP:lib.rs-0080 */ #[cfg_attr(
/* FP:lib.rs-0081 */     feature = "nightly",
/* FP:lib.rs-0082 */     derive(Encodable_NoContext, Decodable_NoContext, HashStable_Generic)
/* FP:lib.rs-0083 */ )]
/* FP:lib.rs-0084 */ pub struct ReprFlags(u8);
/* FP:lib.rs-0085 */ 
/* FP:lib.rs-0086 */ bitflags! {
/* FP:lib.rs-0087 */     impl ReprFlags: u8 {
/* FP:lib.rs-0088 */         const IS_C               = 1 << 0;
/* FP:lib.rs-0089 */         const IS_SIMD            = 1 << 1;
/* FP:lib.rs-0090 */         const IS_TRANSPARENT     = 1 << 2;
/* FP:lib.rs-0091 */         // Internal only for now. If true, don't reorder fields.
/* FP:lib.rs-0092 */         // On its own it does not prevent ABI optimizations.
/* FP:lib.rs-0093 */         const IS_LINEAR          = 1 << 3;
/* FP:lib.rs-0094 */         // If true, the type's crate has opted into layout randomization.
/* FP:lib.rs-0095 */         // Other flags can still inhibit reordering and thus randomization.
/* FP:lib.rs-0096 */         // The seed stored in `ReprOptions.field_shuffle_seed`.
/* FP:lib.rs-0097 */         const RANDOMIZE_LAYOUT   = 1 << 4;
/* FP:lib.rs-0098 */         // Any of these flags being set prevent field reordering optimisation.
/* FP:lib.rs-0099 */         const FIELD_ORDER_UNOPTIMIZABLE   = ReprFlags::IS_C.bits()
/* FP:lib.rs-0100 */                                  | ReprFlags::IS_SIMD.bits()
/* FP:lib.rs-0101 */                                  | ReprFlags::IS_LINEAR.bits();
/* FP:lib.rs-0102 */         const ABI_UNOPTIMIZABLE = ReprFlags::IS_C.bits() | ReprFlags::IS_SIMD.bits();
/* FP:lib.rs-0103 */     }
/* FP:lib.rs-0104 */ }
/* FP:lib.rs-0105 */ 
/* FP:lib.rs-0106 */ // This is the same as `crate::rustc_data_structures::external_bitflags_debug` but without the
/* FP:lib.rs-0107 */ // `rustc_data_structures` to make it build on stable.
/* FP:lib.rs-0108 */ impl std::fmt::Debug for ReprFlags {
/* FP:lib.rs-0109 */     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
/* FP:lib.rs-0110 */         bitflags::parser::to_writer(self, f)
/* FP:lib.rs-0111 */     }
/* FP:lib.rs-0112 */ }
/* FP:lib.rs-0113 */ 
/* FP:lib.rs-0114 */ #[derive(Copy, Clone, Debug, Eq, PartialEq)]
/* FP:lib.rs-0115 */ #[cfg_attr(
/* FP:lib.rs-0116 */     feature = "nightly",
/* FP:lib.rs-0117 */     derive(Encodable_NoContext, Decodable_NoContext, HashStable_Generic)
/* FP:lib.rs-0118 */ )]
/* FP:lib.rs-0119 */ pub enum IntegerType {
/* FP:lib.rs-0120 */     /// Pointer-sized integer type, i.e. `isize` and `usize`. The field shows signedness, e.g.
/* FP:lib.rs-0121 */     /// `Pointer(true)` means `isize`.
/* FP:lib.rs-0122 */     Pointer(bool),
/* FP:lib.rs-0123 */     /// Fixed-sized integer type, e.g. `i8`, `u32`, `i128`. The bool field shows signedness, e.g.
/* FP:lib.rs-0124 */     /// `Fixed(I8, false)` means `u8`.
/* FP:lib.rs-0125 */     Fixed(Integer, bool),
/* FP:lib.rs-0126 */ }
/* FP:lib.rs-0127 */ 
/* FP:lib.rs-0128 */ impl IntegerType {
/* FP:lib.rs-0129 */     pub fn is_signed(&self) -> bool {
/* FP:lib.rs-0130 */         match self {
/* FP:lib.rs-0131 */             IntegerType::Pointer(b) => *b,
/* FP:lib.rs-0132 */             IntegerType::Fixed(_, b) => *b,
/* FP:lib.rs-0133 */         }
/* FP:lib.rs-0134 */     }
/* FP:lib.rs-0135 */ }
/* FP:lib.rs-0136 */ 
/* FP:lib.rs-0137 */ /// Represents the repr options provided by the user.
/* FP:lib.rs-0138 */ #[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
/* FP:lib.rs-0139 */ #[cfg_attr(
/* FP:lib.rs-0140 */     feature = "nightly",
/* FP:lib.rs-0141 */     derive(Encodable_NoContext, Decodable_NoContext, HashStable_Generic)
/* FP:lib.rs-0142 */ )]
/* FP:lib.rs-0143 */ pub struct ReprOptions {
/* FP:lib.rs-0144 */     pub int: Option<IntegerType>,
/* FP:lib.rs-0145 */     pub align: Option<Align>,
/* FP:lib.rs-0146 */     pub pack: Option<Align>,
/* FP:lib.rs-0147 */     pub flags: ReprFlags,
/* FP:lib.rs-0148 */     /// The seed to be used for randomizing a type's layout
/* FP:lib.rs-0149 */     ///
/* FP:lib.rs-0150 */     /// Note: This could technically be a `u128` which would
/* FP:lib.rs-0151 */     /// be the "most accurate" hash as it'd encompass the item and crate
/* FP:lib.rs-0152 */     /// hash without loss, but it does pay the price of being larger.
/* FP:lib.rs-0153 */     /// Everything's a tradeoff, a 64-bit seed should be sufficient for our
/* FP:lib.rs-0154 */     /// purposes (primarily `-Z randomize-layout`)
/* FP:lib.rs-0155 */     pub field_shuffle_seed: Hash64,
/* FP:lib.rs-0156 */ }
/* FP:lib.rs-0157 */ 
/* FP:lib.rs-0158 */ impl ReprOptions {
/* FP:lib.rs-0159 */     #[inline]
/* FP:lib.rs-0160 */     pub fn simd(&self) -> bool {
/* FP:lib.rs-0161 */         self.flags.contains(ReprFlags::IS_SIMD)
/* FP:lib.rs-0162 */     }
/* FP:lib.rs-0163 */ 
/* FP:lib.rs-0164 */     #[inline]
/* FP:lib.rs-0165 */     pub fn c(&self) -> bool {
/* FP:lib.rs-0166 */         self.flags.contains(ReprFlags::IS_C)
/* FP:lib.rs-0167 */     }
/* FP:lib.rs-0168 */ 
/* FP:lib.rs-0169 */     #[inline]
/* FP:lib.rs-0170 */     pub fn packed(&self) -> bool {
/* FP:lib.rs-0171 */         self.pack.is_some()
/* FP:lib.rs-0172 */     }
/* FP:lib.rs-0173 */ 
/* FP:lib.rs-0174 */     #[inline]
/* FP:lib.rs-0175 */     pub fn transparent(&self) -> bool {
/* FP:lib.rs-0176 */         self.flags.contains(ReprFlags::IS_TRANSPARENT)
/* FP:lib.rs-0177 */     }
/* FP:lib.rs-0178 */ 
/* FP:lib.rs-0179 */     #[inline]
/* FP:lib.rs-0180 */     pub fn linear(&self) -> bool {
/* FP:lib.rs-0181 */         self.flags.contains(ReprFlags::IS_LINEAR)
/* FP:lib.rs-0182 */     }
/* FP:lib.rs-0183 */ 
/* FP:lib.rs-0184 */     /// Returns the discriminant type, given these `repr` options.
/* FP:lib.rs-0185 */     /// This must only be called on enums!
/* FP:lib.rs-0186 */     pub fn discr_type(&self) -> IntegerType {
/* FP:lib.rs-0187 */         self.int.unwrap_or(IntegerType::Pointer(true))
/* FP:lib.rs-0188 */     }
/* FP:lib.rs-0189 */ 
/* FP:lib.rs-0190 */     /// Returns `true` if this `#[repr()]` should inhabit "smart enum
/* FP:lib.rs-0191 */     /// layout" optimizations, such as representing `Foo<&T>` as a
/* FP:lib.rs-0192 */     /// single pointer.
/* FP:lib.rs-0193 */     pub fn inhibit_enum_layout_opt(&self) -> bool {
/* FP:lib.rs-0194 */         self.c() || self.int.is_some()
/* FP:lib.rs-0195 */     }
/* FP:lib.rs-0196 */ 
/* FP:lib.rs-0197 */     pub fn inhibit_newtype_abi_optimization(&self) -> bool {
/* FP:lib.rs-0198 */         self.flags.intersects(ReprFlags::ABI_UNOPTIMIZABLE)
/* FP:lib.rs-0199 */     }
/* FP:lib.rs-0200 */ 
/* FP:lib.rs-0201 */     /// Returns `true` if this `#[repr()]` guarantees a fixed field order,
/* FP:lib.rs-0202 */     /// e.g. `repr(C)` or `repr(<int>)`.
/* FP:lib.rs-0203 */     pub fn inhibit_struct_field_reordering(&self) -> bool {
/* FP:lib.rs-0204 */         self.flags.intersects(ReprFlags::FIELD_ORDER_UNOPTIMIZABLE) || self.int.is_some()
/* FP:lib.rs-0205 */     }
/* FP:lib.rs-0206 */ 
/* FP:lib.rs-0207 */     /// Returns `true` if this type is valid for reordering and `-Z randomize-layout`
/* FP:lib.rs-0208 */     /// was enabled for its declaration crate.
/* FP:lib.rs-0209 */     pub fn can_randomize_type_layout(&self) -> bool {
/* FP:lib.rs-0210 */         !self.inhibit_struct_field_reordering() && self.flags.contains(ReprFlags::RANDOMIZE_LAYOUT)
/* FP:lib.rs-0211 */     }
/* FP:lib.rs-0212 */ 
/* FP:lib.rs-0213 */     /// Returns `true` if this `#[repr()]` should inhibit union ABI optimisations.
/* FP:lib.rs-0214 */     pub fn inhibits_union_abi_opt(&self) -> bool {
/* FP:lib.rs-0215 */         self.c()
/* FP:lib.rs-0216 */     }
/* FP:lib.rs-0217 */ }
/* FP:lib.rs-0218 */ 
/* FP:lib.rs-0219 */ /// The maximum supported number of lanes in a SIMD vector.
/* FP:lib.rs-0220 */ ///
/* FP:lib.rs-0221 */ /// This value is selected based on backend support:
/* FP:lib.rs-0222 */ /// * LLVM does not appear to have a vector width limit.
/* FP:lib.rs-0223 */ /// * Cranelift stores the base-2 log of the lane count in a 4 bit integer.
/* FP:lib.rs-0224 */ pub const MAX_SIMD_LANES: u64 = 1 << 0xF;
/* FP:lib.rs-0225 */ 
/* FP:lib.rs-0226 */ /// How pointers are represented in a given address space
/* FP:lib.rs-0227 */ #[derive(Copy, Clone, Debug, PartialEq, Eq)]
/* FP:lib.rs-0228 */ pub struct PointerSpec {
/* FP:lib.rs-0229 */     /// The size of the bitwise representation of the pointer.
/* FP:lib.rs-0230 */     pointer_size: Size,
/* FP:lib.rs-0231 */     /// The alignment of pointers for this address space
/* FP:lib.rs-0232 */     pointer_align: AbiAlign,
/* FP:lib.rs-0233 */     /// The size of the value a pointer can be offset by in this address space.
/* FP:lib.rs-0234 */     pointer_offset: Size,
/* FP:lib.rs-0235 */     /// Pointers into this address space contain extra metadata
/* FP:lib.rs-0236 */     /// FIXME(workingjubilee): Consider adequately reflecting this in the compiler?
/* FP:lib.rs-0237 */     _is_fat: bool,
/* FP:lib.rs-0238 */ }
/* FP:lib.rs-0239 */ 
/* FP:lib.rs-0240 */ /// Parsed [Data layout](https://llvm.org/docs/LangRef.html#data-layout)
/* FP:lib.rs-0241 */ /// for a target, which contains everything needed to compute layouts.
/* FP:lib.rs-0242 */ #[derive(Debug, PartialEq, Eq)]
/* FP:lib.rs-0243 */ pub struct TargetDataLayout {
/* FP:lib.rs-0244 */     pub endian: Endian,
/* FP:lib.rs-0245 */     pub i1_align: AbiAlign,
/* FP:lib.rs-0246 */     pub i8_align: AbiAlign,
/* FP:lib.rs-0247 */     pub i16_align: AbiAlign,
/* FP:lib.rs-0248 */     pub i32_align: AbiAlign,
/* FP:lib.rs-0249 */     pub i64_align: AbiAlign,
/* FP:lib.rs-0250 */     pub i128_align: AbiAlign,
/* FP:lib.rs-0251 */     pub f16_align: AbiAlign,
/* FP:lib.rs-0252 */     pub f32_align: AbiAlign,
/* FP:lib.rs-0253 */     pub f64_align: AbiAlign,
/* FP:lib.rs-0254 */     pub f128_align: AbiAlign,
/* FP:lib.rs-0255 */     pub aggregate_align: AbiAlign,
/* FP:lib.rs-0256 */ 
/* FP:lib.rs-0257 */     /// Alignments for vector types.
/* FP:lib.rs-0258 */     pub vector_align: Vec<(Size, AbiAlign)>,
/* FP:lib.rs-0259 */ 
/* FP:lib.rs-0260 */     pub default_address_space: AddressSpace,
/* FP:lib.rs-0261 */     pub default_address_space_pointer_spec: PointerSpec,
/* FP:lib.rs-0262 */ 
/* FP:lib.rs-0263 */     /// Address space information of all known address spaces.
/* FP:lib.rs-0264 */     ///
/* FP:lib.rs-0265 */     /// # Note
/* FP:lib.rs-0266 */     ///
/* FP:lib.rs-0267 */     /// This vector does not contain the [`PointerSpec`] relative to the default address space,
/* FP:lib.rs-0268 */     /// which instead lives in [`Self::default_address_space_pointer_spec`].
/* FP:lib.rs-0269 */     address_space_info: Vec<(AddressSpace, PointerSpec)>,
/* FP:lib.rs-0270 */ 
/* FP:lib.rs-0271 */     pub instruction_address_space: AddressSpace,
/* FP:lib.rs-0272 */ 
/* FP:lib.rs-0273 */     /// Minimum size of #[repr(C)] enums (default c_int::BITS, usually 32)
/* FP:lib.rs-0274 */     /// Note: This isn't in LLVM's data layout string, it is `short_enum`
/* FP:lib.rs-0275 */     /// so the only valid spec for LLVM is c_int::BITS or 8
/* FP:lib.rs-0276 */     pub c_enum_min_size: Integer,
/* FP:lib.rs-0277 */ }
/* FP:lib.rs-0278 */ 
/* FP:lib.rs-0279 */ impl Default for TargetDataLayout {
/* FP:lib.rs-0280 */     /// Creates an instance of `TargetDataLayout`.
/* FP:lib.rs-0281 */     fn default() -> TargetDataLayout {
/* FP:lib.rs-0282 */         let align = |bits| Align::from_bits(bits).unwrap();
/* FP:lib.rs-0283 */         TargetDataLayout {
/* FP:lib.rs-0284 */             endian: Endian::Big,
/* FP:lib.rs-0285 */             i1_align: AbiAlign::new(align(8)),
/* FP:lib.rs-0286 */             i8_align: AbiAlign::new(align(8)),
/* FP:lib.rs-0287 */             i16_align: AbiAlign::new(align(16)),
/* FP:lib.rs-0288 */             i32_align: AbiAlign::new(align(32)),
/* FP:lib.rs-0289 */             i64_align: AbiAlign::new(align(32)),
/* FP:lib.rs-0290 */             i128_align: AbiAlign::new(align(32)),
/* FP:lib.rs-0291 */             f16_align: AbiAlign::new(align(16)),
/* FP:lib.rs-0292 */             f32_align: AbiAlign::new(align(32)),
/* FP:lib.rs-0293 */             f64_align: AbiAlign::new(align(64)),
/* FP:lib.rs-0294 */             f128_align: AbiAlign::new(align(128)),
/* FP:lib.rs-0295 */             aggregate_align: AbiAlign { abi: align(8) },
/* FP:lib.rs-0296 */             vector_align: vec![
/* FP:lib.rs-0297 */                 (Size::from_bits(64), AbiAlign::new(align(64))),
/* FP:lib.rs-0298 */                 (Size::from_bits(128), AbiAlign::new(align(128))),
/* FP:lib.rs-0299 */             ],
/* FP:lib.rs-0300 */             default_address_space: AddressSpace::ZERO,
/* FP:lib.rs-0301 */             default_address_space_pointer_spec: PointerSpec {
/* FP:lib.rs-0302 */                 pointer_size: Size::from_bits(64),
/* FP:lib.rs-0303 */                 pointer_align: AbiAlign::new(align(64)),
/* FP:lib.rs-0304 */                 pointer_offset: Size::from_bits(64),
/* FP:lib.rs-0305 */                 _is_fat: false,
/* FP:lib.rs-0306 */             },
/* FP:lib.rs-0307 */             address_space_info: vec![],
/* FP:lib.rs-0308 */             instruction_address_space: AddressSpace::ZERO,
/* FP:lib.rs-0309 */             c_enum_min_size: Integer::I32,
/* FP:lib.rs-0310 */         }
/* FP:lib.rs-0311 */     }
/* FP:lib.rs-0312 */ }
/* FP:lib.rs-0313 */ 
/* FP:lib.rs-0314 */ pub enum TargetDataLayoutErrors<'a> {
/* FP:lib.rs-0315 */     InvalidAddressSpace { addr_space: &'a str, cause: &'a str, err: ParseIntError },
/* FP:lib.rs-0316 */     InvalidBits { kind: &'a str, bit: &'a str, cause: &'a str, err: ParseIntError },
/* FP:lib.rs-0317 */     MissingAlignment { cause: &'a str },
/* FP:lib.rs-0318 */     InvalidAlignment { cause: &'a str, err: AlignFromBytesError },
/* FP:lib.rs-0319 */     InconsistentTargetArchitecture { dl: &'a str, target: &'a str },
/* FP:lib.rs-0320 */     InconsistentTargetPointerWidth { pointer_size: u64, target: u16 },
/* FP:lib.rs-0321 */     InvalidBitsSize { err: String },
/* FP:lib.rs-0322 */     UnknownPointerSpecification { err: String },
/* FP:lib.rs-0323 */ }
/* FP:lib.rs-0324 */ 
/* FP:lib.rs-0325 */ impl TargetDataLayout {
/* FP:lib.rs-0326 */     /// Parse data layout from an
/* FP:lib.rs-0327 */     /// [llvm data layout string](https://llvm.org/docs/LangRef.html#data-layout)
/* FP:lib.rs-0328 */     ///
/* FP:lib.rs-0329 */     /// This function doesn't fill `c_enum_min_size` and it will always be `I32` since it can not be
/* FP:lib.rs-0330 */     /// determined from llvm string.
/* FP:lib.rs-0331 */     pub fn parse_from_llvm_datalayout_string<'a>(
/* FP:lib.rs-0332 */         input: &'a str,
/* FP:lib.rs-0333 */         default_address_space: AddressSpace,
/* FP:lib.rs-0334 */     ) -> Result<TargetDataLayout, TargetDataLayoutErrors<'a>> {
/* FP:lib.rs-0335 */         // Parse an address space index from a string.
/* FP:lib.rs-0336 */         let parse_address_space = |s: &'a str, cause: &'a str| {
/* FP:lib.rs-0337 */             s.parse::<u32>().map(AddressSpace).map_err(|err| {
/* FP:lib.rs-0338 */                 TargetDataLayoutErrors::InvalidAddressSpace { addr_space: s, cause, err }
/* FP:lib.rs-0339 */             })
/* FP:lib.rs-0340 */         };
/* FP:lib.rs-0341 */ 
/* FP:lib.rs-0342 */         // Parse a bit count from a string.
/* FP:lib.rs-0343 */         let parse_bits = |s: &'a str, kind: &'a str, cause: &'a str| {
/* FP:lib.rs-0344 */             s.parse::<u64>().map_err(|err| TargetDataLayoutErrors::InvalidBits {
/* FP:lib.rs-0345 */                 kind,
/* FP:lib.rs-0346 */                 bit: s,
/* FP:lib.rs-0347 */                 cause,
/* FP:lib.rs-0348 */                 err,
/* FP:lib.rs-0349 */             })
/* FP:lib.rs-0350 */         };
/* FP:lib.rs-0351 */ 
/* FP:lib.rs-0352 */         // Parse a size string.
/* FP:lib.rs-0353 */         let parse_size =
/* FP:lib.rs-0354 */             |s: &'a str, cause: &'a str| parse_bits(s, "size", cause).map(Size::from_bits);
/* FP:lib.rs-0355 */ 
/* FP:lib.rs-0356 */         // Parse an alignment string.
/* FP:lib.rs-0357 */         let parse_align_str = |s: &'a str, cause: &'a str| {
/* FP:lib.rs-0358 */             let align_from_bits = |bits| {
/* FP:lib.rs-0359 */                 Align::from_bits(bits)
/* FP:lib.rs-0360 */                     .map_err(|err| TargetDataLayoutErrors::InvalidAlignment { cause, err })
/* FP:lib.rs-0361 */             };
/* FP:lib.rs-0362 */             let abi = parse_bits(s, "alignment", cause)?;
/* FP:lib.rs-0363 */             Ok(AbiAlign::new(align_from_bits(abi)?))
/* FP:lib.rs-0364 */         };
/* FP:lib.rs-0365 */ 
/* FP:lib.rs-0366 */         // Parse an alignment sequence, possibly in the form `<align>[:<preferred_alignment>]`,
/* FP:lib.rs-0367 */         // ignoring the secondary alignment specifications.
/* FP:lib.rs-0368 */         let parse_align_seq = |s: &[&'a str], cause: &'a str| {
/* FP:lib.rs-0369 */             if s.is_empty() {
/* FP:lib.rs-0370 */                 return Err(TargetDataLayoutErrors::MissingAlignment { cause });
/* FP:lib.rs-0371 */             }
/* FP:lib.rs-0372 */             parse_align_str(s[0], cause)
/* FP:lib.rs-0373 */         };
/* FP:lib.rs-0374 */ 
/* FP:lib.rs-0375 */         let mut dl = TargetDataLayout::default();
/* FP:lib.rs-0376 */         dl.default_address_space = default_address_space;
/* FP:lib.rs-0377 */ 
/* FP:lib.rs-0378 */         let mut i128_align_src = 64;
/* FP:lib.rs-0379 */         for spec in input.split('-') {
/* FP:lib.rs-0380 */             let spec_parts = spec.split(':').collect::<Vec<_>>();
/* FP:lib.rs-0381 */ 
/* FP:lib.rs-0382 */             match &*spec_parts {
/* FP:lib.rs-0383 */                 ["e"] => dl.endian = Endian::Little,
/* FP:lib.rs-0384 */                 ["E"] => dl.endian = Endian::Big,
/* FP:lib.rs-0385 */                 [p] if p.starts_with('P') => {
/* FP:lib.rs-0386 */                     dl.instruction_address_space = parse_address_space(&p[1..], "P")?
/* FP:lib.rs-0387 */                 }
/* FP:lib.rs-0388 */                 ["a", a @ ..] => dl.aggregate_align = parse_align_seq(a, "a")?,
/* FP:lib.rs-0389 */                 ["f16", a @ ..] => dl.f16_align = parse_align_seq(a, "f16")?,
/* FP:lib.rs-0390 */                 ["f32", a @ ..] => dl.f32_align = parse_align_seq(a, "f32")?,
/* FP:lib.rs-0391 */                 ["f64", a @ ..] => dl.f64_align = parse_align_seq(a, "f64")?,
/* FP:lib.rs-0392 */                 ["f128", a @ ..] => dl.f128_align = parse_align_seq(a, "f128")?,
/* FP:lib.rs-0393 */                 [p, s, a @ ..] if p.starts_with("p") => {
/* FP:lib.rs-0394 */                     let mut p = p.strip_prefix('p').unwrap();
/* FP:lib.rs-0395 */                     let mut _is_fat = false;
/* FP:lib.rs-0396 */ 
/* FP:lib.rs-0397 */                     // Some targets, such as CHERI, use the 'f' suffix in the p- spec to signal that
/* FP:lib.rs-0398 */                     // they use 'fat' pointers. The resulting prefix may look like `pf<addr_space>`.
/* FP:lib.rs-0399 */ 
/* FP:lib.rs-0400 */                     if p.starts_with('f') {
/* FP:lib.rs-0401 */                         p = p.strip_prefix('f').unwrap();
/* FP:lib.rs-0402 */                         _is_fat = true;
/* FP:lib.rs-0403 */                     }
/* FP:lib.rs-0404 */ 
/* FP:lib.rs-0405 */                     // However, we currently don't take into account further specifications:
/* FP:lib.rs-0406 */                     // an error is emitted instead.
/* FP:lib.rs-0407 */                     if p.starts_with(char::is_alphabetic) {
/* FP:lib.rs-0408 */                         return Err(TargetDataLayoutErrors::UnknownPointerSpecification {
/* FP:lib.rs-0409 */                             err: p.to_string(),
/* FP:lib.rs-0410 */                         });
/* FP:lib.rs-0411 */                     }
/* FP:lib.rs-0412 */ 
/* FP:lib.rs-0413 */                     let addr_space = if !p.is_empty() {
/* FP:lib.rs-0414 */                         parse_address_space(p, "p-")?
/* FP:lib.rs-0415 */                     } else {
/* FP:lib.rs-0416 */                         AddressSpace::ZERO
/* FP:lib.rs-0417 */                     };
/* FP:lib.rs-0418 */ 
/* FP:lib.rs-0419 */                     let pointer_size = parse_size(s, "p-")?;
/* FP:lib.rs-0420 */                     let pointer_align = parse_align_seq(a, "p-")?;
/* FP:lib.rs-0421 */                     let info = PointerSpec {
/* FP:lib.rs-0422 */                         pointer_offset: pointer_size,
/* FP:lib.rs-0423 */                         pointer_size,
/* FP:lib.rs-0424 */                         pointer_align,
/* FP:lib.rs-0425 */                         _is_fat,
/* FP:lib.rs-0426 */                     };
/* FP:lib.rs-0427 */                     if addr_space == default_address_space {
/* FP:lib.rs-0428 */                         dl.default_address_space_pointer_spec = info;
/* FP:lib.rs-0429 */                     } else {
/* FP:lib.rs-0430 */                         match dl.address_space_info.iter_mut().find(|(a, _)| *a == addr_space) {
/* FP:lib.rs-0431 */                             Some(e) => e.1 = info,
/* FP:lib.rs-0432 */                             None => {
/* FP:lib.rs-0433 */                                 dl.address_space_info.push((addr_space, info));
/* FP:lib.rs-0434 */                             }
/* FP:lib.rs-0435 */                         }
/* FP:lib.rs-0436 */                     }
/* FP:lib.rs-0437 */                 }
/* FP:lib.rs-0438 */                 [p, s, a, _pr, i] if p.starts_with("p") => {
/* FP:lib.rs-0439 */                     let mut p = p.strip_prefix('p').unwrap();
/* FP:lib.rs-0440 */                     let mut _is_fat = false;
/* FP:lib.rs-0441 */ 
/* FP:lib.rs-0442 */                     // Some targets, such as CHERI, use the 'f' suffix in the p- spec to signal that
/* FP:lib.rs-0443 */                     // they use 'fat' pointers. The resulting prefix may look like `pf<addr_space>`.
/* FP:lib.rs-0444 */ 
/* FP:lib.rs-0445 */                     if p.starts_with('f') {
/* FP:lib.rs-0446 */                         p = p.strip_prefix('f').unwrap();
/* FP:lib.rs-0447 */                         _is_fat = true;
/* FP:lib.rs-0448 */                     }
/* FP:lib.rs-0449 */ 
/* FP:lib.rs-0450 */                     // However, we currently don't take into account further specifications:
/* FP:lib.rs-0451 */                     // an error is emitted instead.
/* FP:lib.rs-0452 */                     if p.starts_with(char::is_alphabetic) {
/* FP:lib.rs-0453 */                         return Err(TargetDataLayoutErrors::UnknownPointerSpecification {
/* FP:lib.rs-0454 */                             err: p.to_string(),
/* FP:lib.rs-0455 */                         });
/* FP:lib.rs-0456 */                     }
/* FP:lib.rs-0457 */ 
/* FP:lib.rs-0458 */                     let addr_space = if !p.is_empty() {
/* FP:lib.rs-0459 */                         parse_address_space(p, "p")?
/* FP:lib.rs-0460 */                     } else {
/* FP:lib.rs-0461 */                         AddressSpace::ZERO
/* FP:lib.rs-0462 */                     };
/* FP:lib.rs-0463 */ 
/* FP:lib.rs-0464 */                     let info = PointerSpec {
/* FP:lib.rs-0465 */                         pointer_size: parse_size(s, "p-")?,
/* FP:lib.rs-0466 */                         pointer_align: parse_align_str(a, "p-")?,
/* FP:lib.rs-0467 */                         pointer_offset: parse_size(i, "p-")?,
/* FP:lib.rs-0468 */                         _is_fat,
/* FP:lib.rs-0469 */                     };
/* FP:lib.rs-0470 */ 
/* FP:lib.rs-0471 */                     if addr_space == default_address_space {
/* FP:lib.rs-0472 */                         dl.default_address_space_pointer_spec = info;
/* FP:lib.rs-0473 */                     } else {
/* FP:lib.rs-0474 */                         match dl.address_space_info.iter_mut().find(|(a, _)| *a == addr_space) {
/* FP:lib.rs-0475 */                             Some(e) => e.1 = info,
/* FP:lib.rs-0476 */                             None => {
/* FP:lib.rs-0477 */                                 dl.address_space_info.push((addr_space, info));
/* FP:lib.rs-0478 */                             }
/* FP:lib.rs-0479 */                         }
/* FP:lib.rs-0480 */                     }
/* FP:lib.rs-0481 */                 }
/* FP:lib.rs-0482 */ 
/* FP:lib.rs-0483 */                 [s, a @ ..] if s.starts_with('i') => {
/* FP:lib.rs-0484 */                     let Ok(bits) = s[1..].parse::<u64>() else {
/* FP:lib.rs-0485 */                         parse_size(&s[1..], "i")?; // For the user error.
/* FP:lib.rs-0486 */                         continue;
/* FP:lib.rs-0487 */                     };
/* FP:lib.rs-0488 */                     let a = parse_align_seq(a, s)?;
/* FP:lib.rs-0489 */                     match bits {
/* FP:lib.rs-0490 */                         1 => dl.i1_align = a,
/* FP:lib.rs-0491 */                         8 => dl.i8_align = a,
/* FP:lib.rs-0492 */                         16 => dl.i16_align = a,
/* FP:lib.rs-0493 */                         32 => dl.i32_align = a,
/* FP:lib.rs-0494 */                         64 => dl.i64_align = a,
/* FP:lib.rs-0495 */                         _ => {}
/* FP:lib.rs-0496 */                     }
/* FP:lib.rs-0497 */                     if bits >= i128_align_src && bits <= 128 {
/* FP:lib.rs-0498 */                         // Default alignment for i128 is decided by taking the alignment of
/* FP:lib.rs-0499 */                         // largest-sized i{64..=128}.
/* FP:lib.rs-0500 */                         i128_align_src = bits;
/* FP:lib.rs-0501 */                         dl.i128_align = a;
/* FP:lib.rs-0502 */                     }
/* FP:lib.rs-0503 */                 }
/* FP:lib.rs-0504 */                 [s, a @ ..] if s.starts_with('v') => {
/* FP:lib.rs-0505 */                     let v_size = parse_size(&s[1..], "v")?;
/* FP:lib.rs-0506 */                     let a = parse_align_seq(a, s)?;
/* FP:lib.rs-0507 */                     if let Some(v) = dl.vector_align.iter_mut().find(|v| v.0 == v_size) {
/* FP:lib.rs-0508 */                         v.1 = a;
/* FP:lib.rs-0509 */                         continue;
/* FP:lib.rs-0510 */                     }
/* FP:lib.rs-0511 */                     // No existing entry, add a new one.
/* FP:lib.rs-0512 */                     dl.vector_align.push((v_size, a));
/* FP:lib.rs-0513 */                 }
/* FP:lib.rs-0514 */                 _ => {} // Ignore everything else.
/* FP:lib.rs-0515 */             }
/* FP:lib.rs-0516 */         }
/* FP:lib.rs-0517 */ 
/* FP:lib.rs-0518 */         // Inherit, if not given, address space information for specific LLVM elements from the
/* FP:lib.rs-0519 */         // default data address space.
/* FP:lib.rs-0520 */         if (dl.instruction_address_space != dl.default_address_space)
/* FP:lib.rs-0521 */             && dl
/* FP:lib.rs-0522 */                 .address_space_info
/* FP:lib.rs-0523 */                 .iter()
/* FP:lib.rs-0524 */                 .find(|(a, _)| *a == dl.instruction_address_space)
/* FP:lib.rs-0525 */                 .is_none()
/* FP:lib.rs-0526 */         {
/* FP:lib.rs-0527 */             dl.address_space_info.push((
/* FP:lib.rs-0528 */                 dl.instruction_address_space,
/* FP:lib.rs-0529 */                 dl.default_address_space_pointer_spec.clone(),
/* FP:lib.rs-0530 */             ));
/* FP:lib.rs-0531 */         }
/* FP:lib.rs-0532 */ 
/* FP:lib.rs-0533 */         Ok(dl)
/* FP:lib.rs-0534 */     }
/* FP:lib.rs-0535 */ 
/* FP:lib.rs-0536 */     /// Returns **exclusive** upper bound on object size in bytes, in the default data address
/* FP:lib.rs-0537 */     /// space.
/* FP:lib.rs-0538 */     ///
/* FP:lib.rs-0539 */     /// The theoretical maximum object size is defined as the maximum positive `isize` value.
/* FP:lib.rs-0540 */     /// This ensures that the `offset` semantics remain well-defined by allowing it to correctly
/* FP:lib.rs-0541 */     /// index every address within an object along with one byte past the end, along with allowing
/* FP:lib.rs-0542 */     /// `isize` to store the difference between any two pointers into an object.
/* FP:lib.rs-0543 */     ///
/* FP:lib.rs-0544 */     /// LLVM uses a 64-bit integer to represent object size in *bits*, but we care only for bytes,
/* FP:lib.rs-0545 */     /// so we adopt such a more-constrained size bound due to its technical limitations.
/* FP:lib.rs-0546 */     #[inline]
/* FP:lib.rs-0547 */     pub fn obj_size_bound(&self) -> u64 {
/* FP:lib.rs-0548 */         match self.pointer_size().bits() {
/* FP:lib.rs-0549 */             16 => 1 << 15,
/* FP:lib.rs-0550 */             32 => 1 << 31,
/* FP:lib.rs-0551 */             64 => 1 << 61,
/* FP:lib.rs-0552 */             bits => panic!("obj_size_bound: unknown pointer bit size {bits}"),
/* FP:lib.rs-0553 */         }
/* FP:lib.rs-0554 */     }
/* FP:lib.rs-0555 */ 
/* FP:lib.rs-0556 */     /// Returns **exclusive** upper bound on object size in bytes.
/* FP:lib.rs-0557 */     ///
/* FP:lib.rs-0558 */     /// The theoretical maximum object size is defined as the maximum positive `isize` value.
/* FP:lib.rs-0559 */     /// This ensures that the `offset` semantics remain well-defined by allowing it to correctly
/* FP:lib.rs-0560 */     /// index every address within an object along with one byte past the end, along with allowing
/* FP:lib.rs-0561 */     /// `isize` to store the difference between any two pointers into an object.
/* FP:lib.rs-0562 */     ///
/* FP:lib.rs-0563 */     /// LLVM uses a 64-bit integer to represent object size in *bits*, but we care only for bytes,
/* FP:lib.rs-0564 */     /// so we adopt such a more-constrained size bound due to its technical limitations.
/* FP:lib.rs-0565 */     #[inline]
/* FP:lib.rs-0566 */     pub fn obj_size_bound_in(&self, address_space: AddressSpace) -> u64 {
/* FP:lib.rs-0567 */         match self.pointer_size_in(address_space).bits() {
/* FP:lib.rs-0568 */             16 => 1 << 15,
/* FP:lib.rs-0569 */             32 => 1 << 31,
/* FP:lib.rs-0570 */             64 => 1 << 61,
/* FP:lib.rs-0571 */             bits => panic!("obj_size_bound: unknown pointer bit size {bits}"),
/* FP:lib.rs-0572 */         }
/* FP:lib.rs-0573 */     }
/* FP:lib.rs-0574 */ 
/* FP:lib.rs-0575 */     #[inline]
/* FP:lib.rs-0576 */     pub fn ptr_sized_integer(&self) -> Integer {
/* FP:lib.rs-0577 */         use Integer::*;
/* FP:lib.rs-0578 */         match self.pointer_offset().bits() {
/* FP:lib.rs-0579 */             16 => I16,
/* FP:lib.rs-0580 */             32 => I32,
/* FP:lib.rs-0581 */             64 => I64,
/* FP:lib.rs-0582 */             bits => panic!("ptr_sized_integer: unknown pointer bit size {bits}"),
/* FP:lib.rs-0583 */         }
/* FP:lib.rs-0584 */     }
/* FP:lib.rs-0585 */ 
/* FP:lib.rs-0586 */     #[inline]
/* FP:lib.rs-0587 */     pub fn ptr_sized_integer_in(&self, address_space: AddressSpace) -> Integer {
/* FP:lib.rs-0588 */         use Integer::*;
/* FP:lib.rs-0589 */         match self.pointer_offset_in(address_space).bits() {
/* FP:lib.rs-0590 */             16 => I16,
/* FP:lib.rs-0591 */             32 => I32,
/* FP:lib.rs-0592 */             64 => I64,
/* FP:lib.rs-0593 */             bits => panic!("ptr_sized_integer: unknown pointer bit size {bits}"),
/* FP:lib.rs-0594 */         }
/* FP:lib.rs-0595 */     }
/* FP:lib.rs-0596 */ 
/* FP:lib.rs-0597 */     /// psABI-mandated alignment for a vector type, if any
/* FP:lib.rs-0598 */     #[inline]
/* FP:lib.rs-0599 */     fn cabi_vector_align(&self, vec_size: Size) -> Option<AbiAlign> {
/* FP:lib.rs-0600 */         self.vector_align
/* FP:lib.rs-0601 */             .iter()
/* FP:lib.rs-0602 */             .find(|(size, _align)| *size == vec_size)
/* FP:lib.rs-0603 */             .map(|(_size, align)| *align)
/* FP:lib.rs-0604 */     }
/* FP:lib.rs-0605 */ 
/* FP:lib.rs-0606 */     /// an alignment resembling the one LLVM would pick for a vector
/* FP:lib.rs-0607 */     #[inline]
/* FP:lib.rs-0608 */     pub fn llvmlike_vector_align(&self, vec_size: Size) -> AbiAlign {
/* FP:lib.rs-0609 */         self.cabi_vector_align(vec_size).unwrap_or(AbiAlign::new(
/* FP:lib.rs-0610 */             Align::from_bytes(vec_size.bytes().next_power_of_two()).unwrap(),
/* FP:lib.rs-0611 */         ))
/* FP:lib.rs-0612 */     }
/* FP:lib.rs-0613 */ 
/* FP:lib.rs-0614 */     /// Get the pointer size in the default data address space.
/* FP:lib.rs-0615 */     #[inline]
/* FP:lib.rs-0616 */     pub fn pointer_size(&self) -> Size {
/* FP:lib.rs-0617 */         self.default_address_space_pointer_spec.pointer_size
/* FP:lib.rs-0618 */     }
/* FP:lib.rs-0619 */ 
/* FP:lib.rs-0620 */     /// Get the pointer size in a specific address space.
/* FP:lib.rs-0621 */     #[inline]
/* FP:lib.rs-0622 */     pub fn pointer_size_in(&self, c: AddressSpace) -> Size {
/* FP:lib.rs-0623 */         if c == self.default_address_space {
/* FP:lib.rs-0624 */             return self.default_address_space_pointer_spec.pointer_size;
/* FP:lib.rs-0625 */         }
/* FP:lib.rs-0626 */ 
/* FP:lib.rs-0627 */         if let Some(e) = self.address_space_info.iter().find(|(a, _)| a == &c) {
/* FP:lib.rs-0628 */             e.1.pointer_size
/* FP:lib.rs-0629 */         } else {
/* FP:lib.rs-0630 */             panic!("Use of unknown address space {c:?}");
/* FP:lib.rs-0631 */         }
/* FP:lib.rs-0632 */     }
/* FP:lib.rs-0633 */ 
/* FP:lib.rs-0634 */     /// Get the pointer index in the default data address space.
/* FP:lib.rs-0635 */     #[inline]
/* FP:lib.rs-0636 */     pub fn pointer_offset(&self) -> Size {
/* FP:lib.rs-0637 */         self.default_address_space_pointer_spec.pointer_offset
/* FP:lib.rs-0638 */     }
/* FP:lib.rs-0639 */ 
/* FP:lib.rs-0640 */     /// Get the pointer index in a specific address space.
/* FP:lib.rs-0641 */     #[inline]
/* FP:lib.rs-0642 */     pub fn pointer_offset_in(&self, c: AddressSpace) -> Size {
/* FP:lib.rs-0643 */         if c == self.default_address_space {
/* FP:lib.rs-0644 */             return self.default_address_space_pointer_spec.pointer_offset;
/* FP:lib.rs-0645 */         }
/* FP:lib.rs-0646 */ 
/* FP:lib.rs-0647 */         if let Some(e) = self.address_space_info.iter().find(|(a, _)| a == &c) {
/* FP:lib.rs-0648 */             e.1.pointer_offset
/* FP:lib.rs-0649 */         } else {
/* FP:lib.rs-0650 */             panic!("Use of unknown address space {c:?}");
/* FP:lib.rs-0651 */         }
/* FP:lib.rs-0652 */     }
/* FP:lib.rs-0653 */ 
/* FP:lib.rs-0654 */     /// Get the pointer alignment in the default data address space.
/* FP:lib.rs-0655 */     #[inline]
/* FP:lib.rs-0656 */     pub fn pointer_align(&self) -> AbiAlign {
/* FP:lib.rs-0657 */         self.default_address_space_pointer_spec.pointer_align
/* FP:lib.rs-0658 */     }
/* FP:lib.rs-0659 */ 
/* FP:lib.rs-0660 */     /// Get the pointer alignment in a specific address space.
/* FP:lib.rs-0661 */     #[inline]
/* FP:lib.rs-0662 */     pub fn pointer_align_in(&self, c: AddressSpace) -> AbiAlign {
/* FP:lib.rs-0663 */         if c == self.default_address_space {
/* FP:lib.rs-0664 */             return self.default_address_space_pointer_spec.pointer_align;
/* FP:lib.rs-0665 */         }
/* FP:lib.rs-0666 */ 
/* FP:lib.rs-0667 */         if let Some(e) = self.address_space_info.iter().find(|(a, _)| a == &c) {
/* FP:lib.rs-0668 */             e.1.pointer_align
/* FP:lib.rs-0669 */         } else {
/* FP:lib.rs-0670 */             panic!("Use of unknown address space {c:?}");
/* FP:lib.rs-0671 */         }
/* FP:lib.rs-0672 */     }
/* FP:lib.rs-0673 */ }
/* FP:lib.rs-0674 */ 
/* FP:lib.rs-0675 */ pub trait HasDataLayout {
/* FP:lib.rs-0676 */     fn data_layout(&self) -> &TargetDataLayout;
/* FP:lib.rs-0677 */ }
/* FP:lib.rs-0678 */ 
/* FP:lib.rs-0679 */ impl HasDataLayout for TargetDataLayout {
/* FP:lib.rs-0680 */     #[inline]
/* FP:lib.rs-0681 */     fn data_layout(&self) -> &TargetDataLayout {
/* FP:lib.rs-0682 */         self
/* FP:lib.rs-0683 */     }
/* FP:lib.rs-0684 */ }
/* FP:lib.rs-0685 */ 
/* FP:lib.rs-0686 */ // used by rust-analyzer
/* FP:lib.rs-0687 */ impl HasDataLayout for &TargetDataLayout {
/* FP:lib.rs-0688 */     #[inline]
/* FP:lib.rs-0689 */     fn data_layout(&self) -> &TargetDataLayout {
/* FP:lib.rs-0690 */         (**self).data_layout()
/* FP:lib.rs-0691 */     }
/* FP:lib.rs-0692 */ }
/* FP:lib.rs-0693 */ 
/* FP:lib.rs-0694 */ /// Endianness of the target, which must match cfg(target-endian).
/* FP:lib.rs-0695 */ #[derive(Copy, Clone, PartialEq, Eq)]
/* FP:lib.rs-0696 */ pub enum Endian {
/* FP:lib.rs-0697 */     Little,
/* FP:lib.rs-0698 */     Big,
/* FP:lib.rs-0699 */ }
/* FP:lib.rs-0700 */ 
/* FP:lib.rs-0701 */ impl Endian {
/* FP:lib.rs-0702 */     pub fn as_str(&self) -> &'static str {
/* FP:lib.rs-0703 */         match self {
/* FP:lib.rs-0704 */             Self::Little => "little",
/* FP:lib.rs-0705 */             Self::Big => "big",
/* FP:lib.rs-0706 */         }
/* FP:lib.rs-0707 */     }
/* FP:lib.rs-0708 */ }
/* FP:lib.rs-0709 */ 
/* FP:lib.rs-0710 */ impl fmt::Debug for Endian {
/* FP:lib.rs-0711 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:lib.rs-0712 */         f.write_str(self.as_str())
/* FP:lib.rs-0713 */     }
/* FP:lib.rs-0714 */ }
/* FP:lib.rs-0715 */ 
/* FP:lib.rs-0716 */ impl FromStr for Endian {
/* FP:lib.rs-0717 */     type Err = String;
/* FP:lib.rs-0718 */ 
/* FP:lib.rs-0719 */     fn from_str(s: &str) -> Result<Self, Self::Err> {
/* FP:lib.rs-0720 */         match s {
/* FP:lib.rs-0721 */             "little" => Ok(Self::Little),
/* FP:lib.rs-0722 */             "big" => Ok(Self::Big),
/* FP:lib.rs-0723 */             _ => Err(format!(r#"unknown endian: "{s}""#)),
/* FP:lib.rs-0724 */         }
/* FP:lib.rs-0725 */     }
/* FP:lib.rs-0726 */ }
/* FP:lib.rs-0727 */ 
/* FP:lib.rs-0728 */ /// Size of a type in bytes.
/* FP:lib.rs-0729 */ #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/* FP:lib.rs-0730 */ #[cfg_attr(
/* FP:lib.rs-0731 */     feature = "nightly",
/* FP:lib.rs-0732 */     derive(Encodable_NoContext, Decodable_NoContext, HashStable_Generic)
/* FP:lib.rs-0733 */ )]
/* FP:lib.rs-0734 */ pub struct Size {
/* FP:lib.rs-0735 */     raw: u64,
/* FP:lib.rs-0736 */ }
/* FP:lib.rs-0737 */ 
/* FP:lib.rs-0738 */ #[cfg(feature = "nightly")]
/* FP:lib.rs-0739 */ impl StableOrd for Size {
/* FP:lib.rs-0740 */     const CAN_USE_UNSTABLE_SORT: bool = true;
/* FP:lib.rs-0741 */ 
/* FP:lib.rs-0742 */     // `Ord` is implemented as just comparing numerical values and numerical values
/* FP:lib.rs-0743 */     // are not changed by (de-)serialization.
/* FP:lib.rs-0744 */     const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED: () = ();
/* FP:lib.rs-0745 */ }
/* FP:lib.rs-0746 */ 
/* FP:lib.rs-0747 */ // This is debug-printed a lot in larger structs, don't waste too much space there
/* FP:lib.rs-0748 */ impl fmt::Debug for Size {
/* FP:lib.rs-0749 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:lib.rs-0750 */         write!(f, "Size({} bytes)", self.bytes())
/* FP:lib.rs-0751 */     }
/* FP:lib.rs-0752 */ }
/* FP:lib.rs-0753 */ 
/* FP:lib.rs-0754 */ impl Size {
/* FP:lib.rs-0755 */     pub const ZERO: Size = Size { raw: 0 };
/* FP:lib.rs-0756 */ 
/* FP:lib.rs-0757 */     /// Rounds `bits` up to the next-higher byte boundary, if `bits` is
/* FP:lib.rs-0758 */     /// not a multiple of 8.
/* FP:lib.rs-0759 */     pub fn from_bits(bits: impl TryInto<u64>) -> Size {
/* FP:lib.rs-0760 */         let bits = bits.try_into().ok().unwrap();
/* FP:lib.rs-0761 */         Size { raw: bits.div_ceil(8) }
/* FP:lib.rs-0762 */     }
/* FP:lib.rs-0763 */ 
/* FP:lib.rs-0764 */     #[inline]
/* FP:lib.rs-0765 */     pub fn from_bytes(bytes: impl TryInto<u64>) -> Size {
/* FP:lib.rs-0766 */         let bytes: u64 = bytes.try_into().ok().unwrap();
/* FP:lib.rs-0767 */         Size { raw: bytes }
/* FP:lib.rs-0768 */     }
/* FP:lib.rs-0769 */ 
/* FP:lib.rs-0770 */     #[inline]
/* FP:lib.rs-0771 */     pub fn bytes(self) -> u64 {
/* FP:lib.rs-0772 */         self.raw
/* FP:lib.rs-0773 */     }
/* FP:lib.rs-0774 */ 
/* FP:lib.rs-0775 */     #[inline]
/* FP:lib.rs-0776 */     pub fn bytes_usize(self) -> usize {
/* FP:lib.rs-0777 */         self.bytes().try_into().unwrap()
/* FP:lib.rs-0778 */     }
/* FP:lib.rs-0779 */ 
/* FP:lib.rs-0780 */     #[inline]
/* FP:lib.rs-0781 */     pub fn bits(self) -> u64 {
/* FP:lib.rs-0782 */         #[cold]
/* FP:lib.rs-0783 */         fn overflow(bytes: u64) -> ! {
/* FP:lib.rs-0784 */             panic!("Size::bits: {bytes} bytes in bits doesn't fit in u64")
/* FP:lib.rs-0785 */         }
/* FP:lib.rs-0786 */ 
/* FP:lib.rs-0787 */         self.bytes().checked_mul(8).unwrap_or_else(|| overflow(self.bytes()))
/* FP:lib.rs-0788 */     }
/* FP:lib.rs-0789 */ 
/* FP:lib.rs-0790 */     #[inline]
/* FP:lib.rs-0791 */     pub fn bits_usize(self) -> usize {
/* FP:lib.rs-0792 */         self.bits().try_into().unwrap()
/* FP:lib.rs-0793 */     }
/* FP:lib.rs-0794 */ 
/* FP:lib.rs-0795 */     #[inline]
/* FP:lib.rs-0796 */     pub fn align_to(self, align: Align) -> Size {
/* FP:lib.rs-0797 */         let mask = align.bytes() - 1;
/* FP:lib.rs-0798 */         Size::from_bytes((self.bytes() + mask) & !mask)
/* FP:lib.rs-0799 */     }
/* FP:lib.rs-0800 */ 
/* FP:lib.rs-0801 */     #[inline]
/* FP:lib.rs-0802 */     pub fn is_aligned(self, align: Align) -> bool {
/* FP:lib.rs-0803 */         let mask = align.bytes() - 1;
/* FP:lib.rs-0804 */         self.bytes() & mask == 0
/* FP:lib.rs-0805 */     }
/* FP:lib.rs-0806 */ 
/* FP:lib.rs-0807 */     #[inline]
/* FP:lib.rs-0808 */     pub fn checked_add<C: HasDataLayout>(self, offset: Size, cx: &C) -> Option<Size> {
/* FP:lib.rs-0809 */         let dl = cx.data_layout();
/* FP:lib.rs-0810 */ 
/* FP:lib.rs-0811 */         let bytes = self.bytes().checked_add(offset.bytes())?;
/* FP:lib.rs-0812 */ 
/* FP:lib.rs-0813 */         if bytes < dl.obj_size_bound() { Some(Size::from_bytes(bytes)) } else { None }
/* FP:lib.rs-0814 */     }
/* FP:lib.rs-0815 */ 
/* FP:lib.rs-0816 */     #[inline]
/* FP:lib.rs-0817 */     pub fn checked_mul<C: HasDataLayout>(self, count: u64, cx: &C) -> Option<Size> {
/* FP:lib.rs-0818 */         let dl = cx.data_layout();
/* FP:lib.rs-0819 */ 
/* FP:lib.rs-0820 */         let bytes = self.bytes().checked_mul(count)?;
/* FP:lib.rs-0821 */         if bytes < dl.obj_size_bound() { Some(Size::from_bytes(bytes)) } else { None }
/* FP:lib.rs-0822 */     }
/* FP:lib.rs-0823 */ 
/* FP:lib.rs-0824 */     /// Truncates `value` to `self` bits and then sign-extends it to 128 bits
/* FP:lib.rs-0825 */     /// (i.e., if it is negative, fill with 1's on the left).
/* FP:lib.rs-0826 */     #[inline]
/* FP:lib.rs-0827 */     pub fn sign_extend(self, value: u128) -> i128 {
/* FP:lib.rs-0828 */         let size = self.bits();
/* FP:lib.rs-0829 */         if size == 0 {
/* FP:lib.rs-0830 */             // Truncated until nothing is left.
/* FP:lib.rs-0831 */             return 0;
/* FP:lib.rs-0832 */         }
/* FP:lib.rs-0833 */         // Sign-extend it.
/* FP:lib.rs-0834 */         let shift = 128 - size;
/* FP:lib.rs-0835 */         // Shift the unsigned value to the left, then shift back to the right as signed
/* FP:lib.rs-0836 */         // (essentially fills with sign bit on the left).
/* FP:lib.rs-0837 */         ((value << shift) as i128) >> shift
/* FP:lib.rs-0838 */     }
/* FP:lib.rs-0839 */ 
/* FP:lib.rs-0840 */     /// Truncates `value` to `self` bits.
/* FP:lib.rs-0841 */     #[inline]
/* FP:lib.rs-0842 */     pub fn truncate(self, value: u128) -> u128 {
/* FP:lib.rs-0843 */         let size = self.bits();
/* FP:lib.rs-0844 */         if size == 0 {
/* FP:lib.rs-0845 */             // Truncated until nothing is left.
/* FP:lib.rs-0846 */             return 0;
/* FP:lib.rs-0847 */         }
/* FP:lib.rs-0848 */         let shift = 128 - size;
/* FP:lib.rs-0849 */         // Truncate (shift left to drop out leftover values, shift right to fill with zeroes).
/* FP:lib.rs-0850 */         (value << shift) >> shift
/* FP:lib.rs-0851 */     }
/* FP:lib.rs-0852 */ 
/* FP:lib.rs-0853 */     #[inline]
/* FP:lib.rs-0854 */     pub fn signed_int_min(&self) -> i128 {
/* FP:lib.rs-0855 */         self.sign_extend(1_u128 << (self.bits() - 1))
/* FP:lib.rs-0856 */     }
/* FP:lib.rs-0857 */ 
/* FP:lib.rs-0858 */     #[inline]
/* FP:lib.rs-0859 */     pub fn signed_int_max(&self) -> i128 {
/* FP:lib.rs-0860 */         i128::MAX >> (128 - self.bits())
/* FP:lib.rs-0861 */     }
/* FP:lib.rs-0862 */ 
/* FP:lib.rs-0863 */     #[inline]
/* FP:lib.rs-0864 */     pub fn unsigned_int_max(&self) -> u128 {
/* FP:lib.rs-0865 */         u128::MAX >> (128 - self.bits())
/* FP:lib.rs-0866 */     }
/* FP:lib.rs-0867 */ }
/* FP:lib.rs-0868 */ 
/* FP:lib.rs-0869 */ // Panicking addition, subtraction and multiplication for convenience.
/* FP:lib.rs-0870 */ // Avoid during layout computation, return `LayoutError` instead.
/* FP:lib.rs-0871 */ 
/* FP:lib.rs-0872 */ impl Add for Size {
/* FP:lib.rs-0873 */     type Output = Size;
/* FP:lib.rs-0874 */     #[inline]
/* FP:lib.rs-0875 */     fn add(self, other: Size) -> Size {
/* FP:lib.rs-0876 */         Size::from_bytes(self.bytes().checked_add(other.bytes()).unwrap_or_else(|| {
/* FP:lib.rs-0877 */             panic!("Size::add: {} + {} doesn't fit in u64", self.bytes(), other.bytes())
/* FP:lib.rs-0878 */         }))
/* FP:lib.rs-0879 */     }
/* FP:lib.rs-0880 */ }
/* FP:lib.rs-0881 */ 
/* FP:lib.rs-0882 */ impl Sub for Size {
/* FP:lib.rs-0883 */     type Output = Size;
/* FP:lib.rs-0884 */     #[inline]
/* FP:lib.rs-0885 */     fn sub(self, other: Size) -> Size {
/* FP:lib.rs-0886 */         Size::from_bytes(self.bytes().checked_sub(other.bytes()).unwrap_or_else(|| {
/* FP:lib.rs-0887 */             panic!("Size::sub: {} - {} would result in negative size", self.bytes(), other.bytes())
/* FP:lib.rs-0888 */         }))
/* FP:lib.rs-0889 */     }
/* FP:lib.rs-0890 */ }
/* FP:lib.rs-0891 */ 
/* FP:lib.rs-0892 */ impl Mul<Size> for u64 {
/* FP:lib.rs-0893 */     type Output = Size;
/* FP:lib.rs-0894 */     #[inline]
/* FP:lib.rs-0895 */     fn mul(self, size: Size) -> Size {
/* FP:lib.rs-0896 */         size * self
/* FP:lib.rs-0897 */     }
/* FP:lib.rs-0898 */ }
/* FP:lib.rs-0899 */ 
/* FP:lib.rs-0900 */ impl Mul<u64> for Size {
/* FP:lib.rs-0901 */     type Output = Size;
/* FP:lib.rs-0902 */     #[inline]
/* FP:lib.rs-0903 */     fn mul(self, count: u64) -> Size {
/* FP:lib.rs-0904 */         match self.bytes().checked_mul(count) {
/* FP:lib.rs-0905 */             Some(bytes) => Size::from_bytes(bytes),
/* FP:lib.rs-0906 */             None => panic!("Size::mul: {} * {} doesn't fit in u64", self.bytes(), count),
/* FP:lib.rs-0907 */         }
/* FP:lib.rs-0908 */     }
/* FP:lib.rs-0909 */ }
/* FP:lib.rs-0910 */ 
/* FP:lib.rs-0911 */ impl AddAssign for Size {
/* FP:lib.rs-0912 */     #[inline]
/* FP:lib.rs-0913 */     fn add_assign(&mut self, other: Size) {
/* FP:lib.rs-0914 */         *self = *self + other;
/* FP:lib.rs-0915 */     }
/* FP:lib.rs-0916 */ }
/* FP:lib.rs-0917 */ 
/* FP:lib.rs-0918 */ #[cfg(feature = "nightly")]
/* FP:lib.rs-0919 */ impl Step for Size {
/* FP:lib.rs-0920 */     #[inline]
/* FP:lib.rs-0921 */     fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
/* FP:lib.rs-0922 */         u64::steps_between(&start.bytes(), &end.bytes())
/* FP:lib.rs-0923 */     }
/* FP:lib.rs-0924 */ 
/* FP:lib.rs-0925 */     #[inline]
/* FP:lib.rs-0926 */     fn forward_checked(start: Self, count: usize) -> Option<Self> {
/* FP:lib.rs-0927 */         u64::forward_checked(start.bytes(), count).map(Self::from_bytes)
/* FP:lib.rs-0928 */     }
/* FP:lib.rs-0929 */ 
/* FP:lib.rs-0930 */     #[inline]
/* FP:lib.rs-0931 */     fn forward(start: Self, count: usize) -> Self {
/* FP:lib.rs-0932 */         Self::from_bytes(u64::forward(start.bytes(), count))
/* FP:lib.rs-0933 */     }
/* FP:lib.rs-0934 */ 
/* FP:lib.rs-0935 */     #[inline]
/* FP:lib.rs-0936 */     unsafe fn forward_unchecked(start: Self, count: usize) -> Self {
/* FP:lib.rs-0937 */         Self::from_bytes(unsafe { u64::forward_unchecked(start.bytes(), count) })
/* FP:lib.rs-0938 */     }
/* FP:lib.rs-0939 */ 
/* FP:lib.rs-0940 */     #[inline]
/* FP:lib.rs-0941 */     fn backward_checked(start: Self, count: usize) -> Option<Self> {
/* FP:lib.rs-0942 */         u64::backward_checked(start.bytes(), count).map(Self::from_bytes)
/* FP:lib.rs-0943 */     }
/* FP:lib.rs-0944 */ 
/* FP:lib.rs-0945 */     #[inline]
/* FP:lib.rs-0946 */     fn backward(start: Self, count: usize) -> Self {
/* FP:lib.rs-0947 */         Self::from_bytes(u64::backward(start.bytes(), count))
/* FP:lib.rs-0948 */     }
/* FP:lib.rs-0949 */ 
/* FP:lib.rs-0950 */     #[inline]
/* FP:lib.rs-0951 */     unsafe fn backward_unchecked(start: Self, count: usize) -> Self {
/* FP:lib.rs-0952 */         Self::from_bytes(unsafe { u64::backward_unchecked(start.bytes(), count) })
/* FP:lib.rs-0953 */     }
/* FP:lib.rs-0954 */ }
/* FP:lib.rs-0955 */ 
/* FP:lib.rs-0956 */ /// Alignment of a type in bytes (always a power of two).
/* FP:lib.rs-0957 */ #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/* FP:lib.rs-0958 */ #[cfg_attr(
/* FP:lib.rs-0959 */     feature = "nightly",
/* FP:lib.rs-0960 */     derive(Encodable_NoContext, Decodable_NoContext, HashStable_Generic)
/* FP:lib.rs-0961 */ )]
/* FP:lib.rs-0962 */ pub struct Align {
/* FP:lib.rs-0963 */     pow2: u8,
/* FP:lib.rs-0964 */ }
/* FP:lib.rs-0965 */ 
/* FP:lib.rs-0966 */ // This is debug-printed a lot in larger structs, don't waste too much space there
/* FP:lib.rs-0967 */ impl fmt::Debug for Align {
/* FP:lib.rs-0968 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:lib.rs-0969 */         write!(f, "Align({} bytes)", self.bytes())
/* FP:lib.rs-0970 */     }
/* FP:lib.rs-0971 */ }
/* FP:lib.rs-0972 */ 
/* FP:lib.rs-0973 */ #[derive(Clone, Copy)]
/* FP:lib.rs-0974 */ pub enum AlignFromBytesError {
/* FP:lib.rs-0975 */     NotPowerOfTwo(u64),
/* FP:lib.rs-0976 */     TooLarge(u64),
/* FP:lib.rs-0977 */ }
/* FP:lib.rs-0978 */ 
/* FP:lib.rs-0979 */ impl AlignFromBytesError {
/* FP:lib.rs-0980 */     pub fn diag_ident(self) -> &'static str {
/* FP:lib.rs-0981 */         match self {
/* FP:lib.rs-0982 */             Self::NotPowerOfTwo(_) => "not_power_of_two",
/* FP:lib.rs-0983 */             Self::TooLarge(_) => "too_large",
/* FP:lib.rs-0984 */         }
/* FP:lib.rs-0985 */     }
/* FP:lib.rs-0986 */ 
/* FP:lib.rs-0987 */     pub fn align(self) -> u64 {
/* FP:lib.rs-0988 */         let (Self::NotPowerOfTwo(align) | Self::TooLarge(align)) = self;
/* FP:lib.rs-0989 */         align
/* FP:lib.rs-0990 */     }
/* FP:lib.rs-0991 */ }
/* FP:lib.rs-0992 */ 
/* FP:lib.rs-0993 */ impl fmt::Debug for AlignFromBytesError {
/* FP:lib.rs-0994 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:lib.rs-0995 */         fmt::Display::fmt(self, f)
/* FP:lib.rs-0996 */     }
/* FP:lib.rs-0997 */ }
/* FP:lib.rs-0998 */ 
/* FP:lib.rs-0999 */ impl fmt::Display for AlignFromBytesError {
/* FP:lib.rs-1000 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:lib.rs-1001 */         match self {
/* FP:lib.rs-1002 */             AlignFromBytesError::NotPowerOfTwo(align) => write!(f, "`{align}` is not a power of 2"),
/* FP:lib.rs-1003 */             AlignFromBytesError::TooLarge(align) => write!(f, "`{align}` is too large"),
/* FP:lib.rs-1004 */         }
/* FP:lib.rs-1005 */     }
/* FP:lib.rs-1006 */ }
/* FP:lib.rs-1007 */ 
/* FP:lib.rs-1008 */ impl Align {
/* FP:lib.rs-1009 */     pub const ONE: Align = Align { pow2: 0 };
/* FP:lib.rs-1010 */     pub const EIGHT: Align = Align { pow2: 3 };
/* FP:lib.rs-1011 */     // LLVM has a maximal supported alignment of 2^29, we inherit that.
/* FP:lib.rs-1012 */     pub const MAX: Align = Align { pow2: 29 };
/* FP:lib.rs-1013 */ 
/* FP:lib.rs-1014 */     #[inline]
/* FP:lib.rs-1015 */     pub fn from_bits(bits: u64) -> Result<Align, AlignFromBytesError> {
/* FP:lib.rs-1016 */         Align::from_bytes(Size::from_bits(bits).bytes())
/* FP:lib.rs-1017 */     }
/* FP:lib.rs-1018 */ 
/* FP:lib.rs-1019 */     #[inline]
/* FP:lib.rs-1020 */     pub const fn from_bytes(align: u64) -> Result<Align, AlignFromBytesError> {
/* FP:lib.rs-1021 */         // Treat an alignment of 0 bytes like 1-byte alignment.
/* FP:lib.rs-1022 */         if align == 0 {
/* FP:lib.rs-1023 */             return Ok(Align::ONE);
/* FP:lib.rs-1024 */         }
/* FP:lib.rs-1025 */ 
/* FP:lib.rs-1026 */         #[cold]
/* FP:lib.rs-1027 */         const fn not_power_of_2(align: u64) -> AlignFromBytesError {
/* FP:lib.rs-1028 */             AlignFromBytesError::NotPowerOfTwo(align)
/* FP:lib.rs-1029 */         }
/* FP:lib.rs-1030 */ 
/* FP:lib.rs-1031 */         #[cold]
/* FP:lib.rs-1032 */         const fn too_large(align: u64) -> AlignFromBytesError {
/* FP:lib.rs-1033 */             AlignFromBytesError::TooLarge(align)
/* FP:lib.rs-1034 */         }
/* FP:lib.rs-1035 */ 
/* FP:lib.rs-1036 */         let tz = align.trailing_zeros();
/* FP:lib.rs-1037 */         if align != (1 << tz) {
/* FP:lib.rs-1038 */             return Err(not_power_of_2(align));
/* FP:lib.rs-1039 */         }
/* FP:lib.rs-1040 */ 
/* FP:lib.rs-1041 */         let pow2 = tz as u8;
/* FP:lib.rs-1042 */         if pow2 > Self::MAX.pow2 {
/* FP:lib.rs-1043 */             return Err(too_large(align));
/* FP:lib.rs-1044 */         }
/* FP:lib.rs-1045 */ 
/* FP:lib.rs-1046 */         Ok(Align { pow2 })
/* FP:lib.rs-1047 */     }
/* FP:lib.rs-1048 */ 
/* FP:lib.rs-1049 */     #[inline]
/* FP:lib.rs-1050 */     pub const fn bytes(self) -> u64 {
/* FP:lib.rs-1051 */         1 << self.pow2
/* FP:lib.rs-1052 */     }
/* FP:lib.rs-1053 */ 
/* FP:lib.rs-1054 */     #[inline]
/* FP:lib.rs-1055 */     pub fn bytes_usize(self) -> usize {
/* FP:lib.rs-1056 */         self.bytes().try_into().unwrap()
/* FP:lib.rs-1057 */     }
/* FP:lib.rs-1058 */ 
/* FP:lib.rs-1059 */     #[inline]
/* FP:lib.rs-1060 */     pub const fn bits(self) -> u64 {
/* FP:lib.rs-1061 */         self.bytes() * 8
/* FP:lib.rs-1062 */     }
/* FP:lib.rs-1063 */ 
/* FP:lib.rs-1064 */     #[inline]
/* FP:lib.rs-1065 */     pub fn bits_usize(self) -> usize {
/* FP:lib.rs-1066 */         self.bits().try_into().unwrap()
/* FP:lib.rs-1067 */     }
/* FP:lib.rs-1068 */ 
/* FP:lib.rs-1069 */     /// Obtain the greatest factor of `size` that is an alignment
/* FP:lib.rs-1070 */     /// (the largest power of two the Size is a multiple of).
/* FP:lib.rs-1071 */     ///
/* FP:lib.rs-1072 */     /// Note that all numbers are factors of 0
/* FP:lib.rs-1073 */     #[inline]
/* FP:lib.rs-1074 */     pub fn max_aligned_factor(size: Size) -> Align {
/* FP:lib.rs-1075 */         Align { pow2: size.bytes().trailing_zeros() as u8 }
/* FP:lib.rs-1076 */     }
/* FP:lib.rs-1077 */ 
/* FP:lib.rs-1078 */     /// Reduces Align to an aligned factor of `size`.
/* FP:lib.rs-1079 */     #[inline]
/* FP:lib.rs-1080 */     pub fn restrict_for_offset(self, size: Size) -> Align {
/* FP:lib.rs-1081 */         self.min(Align::max_aligned_factor(size))
/* FP:lib.rs-1082 */     }
/* FP:lib.rs-1083 */ }
/* FP:lib.rs-1084 */ 
/* FP:lib.rs-1085 */ /// A pair of alignments, ABI-mandated and preferred.
/* FP:lib.rs-1086 */ ///
/* FP:lib.rs-1087 */ /// The "preferred" alignment is an LLVM concept that is virtually meaningless to Rust code:
/* FP:lib.rs-1088 */ /// it is not exposed semantically to programmers nor can they meaningfully affect it.
/* FP:lib.rs-1089 */ /// The only concern for us is that preferred alignment must not be less than the mandated alignment
/* FP:lib.rs-1090 */ /// and thus in practice the two values are almost always identical.
/* FP:lib.rs-1091 */ ///
/* FP:lib.rs-1092 */ /// An example of a rare thing actually affected by preferred alignment is aligning of statics.
/* FP:lib.rs-1093 */ /// It is of effectively no consequence for layout in structs and on the stack.
/* FP:lib.rs-1094 */ #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
/* FP:lib.rs-1095 */ #[cfg_attr(feature = "nightly", derive(HashStable_Generic))]
/* FP:lib.rs-1096 */ pub struct AbiAlign {
/* FP:lib.rs-1097 */     pub abi: Align,
/* FP:lib.rs-1098 */ }
/* FP:lib.rs-1099 */ 
/* FP:lib.rs-1100 */ impl AbiAlign {
/* FP:lib.rs-1101 */     #[inline]
/* FP:lib.rs-1102 */     pub fn new(align: Align) -> AbiAlign {
/* FP:lib.rs-1103 */         AbiAlign { abi: align }
/* FP:lib.rs-1104 */     }
/* FP:lib.rs-1105 */ 
/* FP:lib.rs-1106 */     #[inline]
/* FP:lib.rs-1107 */     pub fn min(self, other: AbiAlign) -> AbiAlign {
/* FP:lib.rs-1108 */         AbiAlign { abi: self.abi.min(other.abi) }
/* FP:lib.rs-1109 */     }
/* FP:lib.rs-1110 */ 
/* FP:lib.rs-1111 */     #[inline]
/* FP:lib.rs-1112 */     pub fn max(self, other: AbiAlign) -> AbiAlign {
/* FP:lib.rs-1113 */         AbiAlign { abi: self.abi.max(other.abi) }
/* FP:lib.rs-1114 */     }
/* FP:lib.rs-1115 */ }
/* FP:lib.rs-1116 */ 
/* FP:lib.rs-1117 */ impl Deref for AbiAlign {
/* FP:lib.rs-1118 */     type Target = Align;
/* FP:lib.rs-1119 */ 
/* FP:lib.rs-1120 */     fn deref(&self) -> &Self::Target {
/* FP:lib.rs-1121 */         &self.abi
/* FP:lib.rs-1122 */     }
/* FP:lib.rs-1123 */ }
/* FP:lib.rs-1124 */ 
/* FP:lib.rs-1125 */ /// Integers, also used for enum discriminants.
/* FP:lib.rs-1126 */ #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
/* FP:lib.rs-1127 */ #[cfg_attr(
/* FP:lib.rs-1128 */     feature = "nightly",
/* FP:lib.rs-1129 */     derive(Encodable_NoContext, Decodable_NoContext, HashStable_Generic)
/* FP:lib.rs-1130 */ )]
/* FP:lib.rs-1131 */ pub enum Integer {
/* FP:lib.rs-1132 */     I8,
/* FP:lib.rs-1133 */     I16,
/* FP:lib.rs-1134 */     I32,
/* FP:lib.rs-1135 */     I64,
/* FP:lib.rs-1136 */     I128,
/* FP:lib.rs-1137 */ }
/* FP:lib.rs-1138 */ 
/* FP:lib.rs-1139 */ impl Integer {
/* FP:lib.rs-1140 */     pub fn int_ty_str(self) -> &'static str {
/* FP:lib.rs-1141 */         use Integer::*;
/* FP:lib.rs-1142 */         match self {
/* FP:lib.rs-1143 */             I8 => "i8",
/* FP:lib.rs-1144 */             I16 => "i16",
/* FP:lib.rs-1145 */             I32 => "i32",
/* FP:lib.rs-1146 */             I64 => "i64",
/* FP:lib.rs-1147 */             I128 => "i128",
/* FP:lib.rs-1148 */         }
/* FP:lib.rs-1149 */     }
/* FP:lib.rs-1150 */ 
/* FP:lib.rs-1151 */     pub fn uint_ty_str(self) -> &'static str {
/* FP:lib.rs-1152 */         use Integer::*;
/* FP:lib.rs-1153 */         match self {
/* FP:lib.rs-1154 */             I8 => "u8",
/* FP:lib.rs-1155 */             I16 => "u16",
/* FP:lib.rs-1156 */             I32 => "u32",
/* FP:lib.rs-1157 */             I64 => "u64",
/* FP:lib.rs-1158 */             I128 => "u128",
/* FP:lib.rs-1159 */         }
/* FP:lib.rs-1160 */     }
/* FP:lib.rs-1161 */ 
/* FP:lib.rs-1162 */     #[inline]
/* FP:lib.rs-1163 */     pub fn size(self) -> Size {
/* FP:lib.rs-1164 */         use Integer::*;
/* FP:lib.rs-1165 */         match self {
/* FP:lib.rs-1166 */             I8 => Size::from_bytes(1),
/* FP:lib.rs-1167 */             I16 => Size::from_bytes(2),
/* FP:lib.rs-1168 */             I32 => Size::from_bytes(4),
/* FP:lib.rs-1169 */             I64 => Size::from_bytes(8),
/* FP:lib.rs-1170 */             I128 => Size::from_bytes(16),
/* FP:lib.rs-1171 */         }
/* FP:lib.rs-1172 */     }
/* FP:lib.rs-1173 */ 
/* FP:lib.rs-1174 */     /// Gets the Integer type from an IntegerType.
/* FP:lib.rs-1175 */     pub fn from_attr<C: HasDataLayout>(cx: &C, ity: IntegerType) -> Integer {
/* FP:lib.rs-1176 */         let dl = cx.data_layout();
/* FP:lib.rs-1177 */ 
/* FP:lib.rs-1178 */         match ity {
/* FP:lib.rs-1179 */             IntegerType::Pointer(_) => dl.ptr_sized_integer(),
/* FP:lib.rs-1180 */             IntegerType::Fixed(x, _) => x,
/* FP:lib.rs-1181 */         }
/* FP:lib.rs-1182 */     }
/* FP:lib.rs-1183 */ 
/* FP:lib.rs-1184 */     pub fn align<C: HasDataLayout>(self, cx: &C) -> AbiAlign {
/* FP:lib.rs-1185 */         use Integer::*;
/* FP:lib.rs-1186 */         let dl = cx.data_layout();
/* FP:lib.rs-1187 */ 
/* FP:lib.rs-1188 */         match self {
/* FP:lib.rs-1189 */             I8 => dl.i8_align,
/* FP:lib.rs-1190 */             I16 => dl.i16_align,
/* FP:lib.rs-1191 */             I32 => dl.i32_align,
/* FP:lib.rs-1192 */             I64 => dl.i64_align,
/* FP:lib.rs-1193 */             I128 => dl.i128_align,
/* FP:lib.rs-1194 */         }
/* FP:lib.rs-1195 */     }
/* FP:lib.rs-1196 */ 
/* FP:lib.rs-1197 */     /// Returns the largest signed value that can be represented by this Integer.
/* FP:lib.rs-1198 */     #[inline]
/* FP:lib.rs-1199 */     pub fn signed_max(self) -> i128 {
/* FP:lib.rs-1200 */         use Integer::*;
/* FP:lib.rs-1201 */         match self {
/* FP:lib.rs-1202 */             I8 => i8::MAX as i128,
/* FP:lib.rs-1203 */             I16 => i16::MAX as i128,
/* FP:lib.rs-1204 */             I32 => i32::MAX as i128,
/* FP:lib.rs-1205 */             I64 => i64::MAX as i128,
/* FP:lib.rs-1206 */             I128 => i128::MAX,
/* FP:lib.rs-1207 */         }
/* FP:lib.rs-1208 */     }
/* FP:lib.rs-1209 */ 
/* FP:lib.rs-1210 */     /// Returns the smallest signed value that can be represented by this Integer.
/* FP:lib.rs-1211 */     #[inline]
/* FP:lib.rs-1212 */     pub fn signed_min(self) -> i128 {
/* FP:lib.rs-1213 */         use Integer::*;
/* FP:lib.rs-1214 */         match self {
/* FP:lib.rs-1215 */             I8 => i8::MIN as i128,
/* FP:lib.rs-1216 */             I16 => i16::MIN as i128,
/* FP:lib.rs-1217 */             I32 => i32::MIN as i128,
/* FP:lib.rs-1218 */             I64 => i64::MIN as i128,
/* FP:lib.rs-1219 */             I128 => i128::MIN,
/* FP:lib.rs-1220 */         }
/* FP:lib.rs-1221 */     }
/* FP:lib.rs-1222 */ 
/* FP:lib.rs-1223 */     /// Finds the smallest Integer type which can represent the signed value.
/* FP:lib.rs-1224 */     #[inline]
/* FP:lib.rs-1225 */     pub fn fit_signed(x: i128) -> Integer {
/* FP:lib.rs-1226 */         use Integer::*;
/* FP:lib.rs-1227 */         match x {
/* FP:lib.rs-1228 */             -0x0000_0000_0000_0080..=0x0000_0000_0000_007f => I8,
/* FP:lib.rs-1229 */             -0x0000_0000_0000_8000..=0x0000_0000_0000_7fff => I16,
/* FP:lib.rs-1230 */             -0x0000_0000_8000_0000..=0x0000_0000_7fff_ffff => I32,
/* FP:lib.rs-1231 */             -0x8000_0000_0000_0000..=0x7fff_ffff_ffff_ffff => I64,
/* FP:lib.rs-1232 */             _ => I128,
/* FP:lib.rs-1233 */         }
/* FP:lib.rs-1234 */     }
/* FP:lib.rs-1235 */ 
/* FP:lib.rs-1236 */     /// Finds the smallest Integer type which can represent the unsigned value.
/* FP:lib.rs-1237 */     #[inline]
/* FP:lib.rs-1238 */     pub fn fit_unsigned(x: u128) -> Integer {
/* FP:lib.rs-1239 */         use Integer::*;
/* FP:lib.rs-1240 */         match x {
/* FP:lib.rs-1241 */             0..=0x0000_0000_0000_00ff => I8,
/* FP:lib.rs-1242 */             0..=0x0000_0000_0000_ffff => I16,
/* FP:lib.rs-1243 */             0..=0x0000_0000_ffff_ffff => I32,
/* FP:lib.rs-1244 */             0..=0xffff_ffff_ffff_ffff => I64,
/* FP:lib.rs-1245 */             _ => I128,
/* FP:lib.rs-1246 */         }
/* FP:lib.rs-1247 */     }
/* FP:lib.rs-1248 */ 
/* FP:lib.rs-1249 */     /// Finds the smallest integer with the given alignment.
/* FP:lib.rs-1250 */     pub fn for_align<C: HasDataLayout>(cx: &C, wanted: Align) -> Option<Integer> {
/* FP:lib.rs-1251 */         use Integer::*;
/* FP:lib.rs-1252 */         let dl = cx.data_layout();
/* FP:lib.rs-1253 */ 
/* FP:lib.rs-1254 */         [I8, I16, I32, I64, I128].into_iter().find(|&candidate| {
/* FP:lib.rs-1255 */             wanted == candidate.align(dl).abi && wanted.bytes() == candidate.size().bytes()
/* FP:lib.rs-1256 */         })
/* FP:lib.rs-1257 */     }
/* FP:lib.rs-1258 */ 
/* FP:lib.rs-1259 */     /// Find the largest integer with the given alignment or less.
/* FP:lib.rs-1260 */     pub fn approximate_align<C: HasDataLayout>(cx: &C, wanted: Align) -> Integer {
/* FP:lib.rs-1261 */         use Integer::*;
/* FP:lib.rs-1262 */         let dl = cx.data_layout();
/* FP:lib.rs-1263 */ 
/* FP:lib.rs-1264 */         // FIXME(eddyb) maybe include I128 in the future, when it works everywhere.
/* FP:lib.rs-1265 */         for candidate in [I64, I32, I16] {
/* FP:lib.rs-1266 */             if wanted >= candidate.align(dl).abi && wanted.bytes() >= candidate.size().bytes() {
/* FP:lib.rs-1267 */                 return candidate;
/* FP:lib.rs-1268 */             }
/* FP:lib.rs-1269 */         }
/* FP:lib.rs-1270 */         I8
/* FP:lib.rs-1271 */     }
/* FP:lib.rs-1272 */ 
/* FP:lib.rs-1273 */     // FIXME(eddyb) consolidate this and other methods that find the appropriate
/* FP:lib.rs-1274 */     // `Integer` given some requirements.
/* FP:lib.rs-1275 */     #[inline]
/* FP:lib.rs-1276 */     pub fn from_size(size: Size) -> Result<Self, String> {
/* FP:lib.rs-1277 */         match size.bits() {
/* FP:lib.rs-1278 */             8 => Ok(Integer::I8),
/* FP:lib.rs-1279 */             16 => Ok(Integer::I16),
/* FP:lib.rs-1280 */             32 => Ok(Integer::I32),
/* FP:lib.rs-1281 */             64 => Ok(Integer::I64),
/* FP:lib.rs-1282 */             128 => Ok(Integer::I128),
/* FP:lib.rs-1283 */             _ => Err(format!("rust does not support integers with {} bits", size.bits())),
/* FP:lib.rs-1284 */         }
/* FP:lib.rs-1285 */     }
/* FP:lib.rs-1286 */ }
/* FP:lib.rs-1287 */ 
/* FP:lib.rs-1288 */ /// Floating-point types.
/* FP:lib.rs-1289 */ #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
/* FP:lib.rs-1290 */ #[cfg_attr(feature = "nightly", derive(HashStable_Generic))]
/* FP:lib.rs-1291 */ pub enum Float {
/* FP:lib.rs-1292 */     F16,
/* FP:lib.rs-1293 */     F32,
/* FP:lib.rs-1294 */     F64,
/* FP:lib.rs-1295 */     F128,
/* FP:lib.rs-1296 */ }
/* FP:lib.rs-1297 */ 
/* FP:lib.rs-1298 */ impl Float {
/* FP:lib.rs-1299 */     pub fn size(self) -> Size {
/* FP:lib.rs-1300 */         use Float::*;
/* FP:lib.rs-1301 */ 
/* FP:lib.rs-1302 */         match self {
/* FP:lib.rs-1303 */             F16 => Size::from_bits(16),
/* FP:lib.rs-1304 */             F32 => Size::from_bits(32),
/* FP:lib.rs-1305 */             F64 => Size::from_bits(64),
/* FP:lib.rs-1306 */             F128 => Size::from_bits(128),
/* FP:lib.rs-1307 */         }
/* FP:lib.rs-1308 */     }
/* FP:lib.rs-1309 */ 
/* FP:lib.rs-1310 */     pub fn align<C: HasDataLayout>(self, cx: &C) -> AbiAlign {
/* FP:lib.rs-1311 */         use Float::*;
/* FP:lib.rs-1312 */         let dl = cx.data_layout();
/* FP:lib.rs-1313 */ 
/* FP:lib.rs-1314 */         match self {
/* FP:lib.rs-1315 */             F16 => dl.f16_align,
/* FP:lib.rs-1316 */             F32 => dl.f32_align,
/* FP:lib.rs-1317 */             F64 => dl.f64_align,
/* FP:lib.rs-1318 */             F128 => dl.f128_align,
/* FP:lib.rs-1319 */         }
/* FP:lib.rs-1320 */     }
/* FP:lib.rs-1321 */ }
/* FP:lib.rs-1322 */ 
/* FP:lib.rs-1323 */ /// Fundamental unit of memory access and layout.
/* FP:lib.rs-1324 */ #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
/* FP:lib.rs-1325 */ #[cfg_attr(feature = "nightly", derive(HashStable_Generic))]
/* FP:lib.rs-1326 */ pub enum Primitive {
/* FP:lib.rs-1327 */     /// The `bool` is the signedness of the `Integer` type.
/* FP:lib.rs-1328 */     ///
/* FP:lib.rs-1329 */     /// One would think we would not care about such details this low down,
/* FP:lib.rs-1330 */     /// but some ABIs are described in terms of C types and ISAs where the
/* FP:lib.rs-1331 */     /// integer arithmetic is done on {sign,zero}-extended registers, e.g.
/* FP:lib.rs-1332 */     /// a negative integer passed by zero-extension will appear positive in
/* FP:lib.rs-1333 */     /// the callee, and most operations on it will produce the wrong values.
/* FP:lib.rs-1334 */     Int(Integer, bool),
/* FP:lib.rs-1335 */     Float(Float),
/* FP:lib.rs-1336 */     Pointer(AddressSpace),
/* FP:lib.rs-1337 */ }
/* FP:lib.rs-1338 */ 
/* FP:lib.rs-1339 */ impl Primitive {
/* FP:lib.rs-1340 */     pub fn size<C: HasDataLayout>(self, cx: &C) -> Size {
/* FP:lib.rs-1341 */         use Primitive::*;
/* FP:lib.rs-1342 */         let dl = cx.data_layout();
/* FP:lib.rs-1343 */ 
/* FP:lib.rs-1344 */         match self {
/* FP:lib.rs-1345 */             Int(i, _) => i.size(),
/* FP:lib.rs-1346 */             Float(f) => f.size(),
/* FP:lib.rs-1347 */             Pointer(a) => dl.pointer_size_in(a),
/* FP:lib.rs-1348 */         }
/* FP:lib.rs-1349 */     }
/* FP:lib.rs-1350 */ 
/* FP:lib.rs-1351 */     pub fn align<C: HasDataLayout>(self, cx: &C) -> AbiAlign {
/* FP:lib.rs-1352 */         use Primitive::*;
/* FP:lib.rs-1353 */         let dl = cx.data_layout();
/* FP:lib.rs-1354 */ 
/* FP:lib.rs-1355 */         match self {
/* FP:lib.rs-1356 */             Int(i, _) => i.align(dl),
/* FP:lib.rs-1357 */             Float(f) => f.align(dl),
/* FP:lib.rs-1358 */             Pointer(a) => dl.pointer_align_in(a),
/* FP:lib.rs-1359 */         }
/* FP:lib.rs-1360 */     }
/* FP:lib.rs-1361 */ }
/* FP:lib.rs-1362 */ 
/* FP:lib.rs-1363 */ /// Inclusive wrap-around range of valid values, that is, if
/* FP:lib.rs-1364 */ /// start > end, it represents `start..=MAX`, followed by `0..=end`.
/* FP:lib.rs-1365 */ ///
/* FP:lib.rs-1366 */ /// That is, for an i8 primitive, a range of `254..=2` means following
/* FP:lib.rs-1367 */ /// sequence:
/* FP:lib.rs-1368 */ ///
/* FP:lib.rs-1369 */ ///    254 (-2), 255 (-1), 0, 1, 2
/* FP:lib.rs-1370 */ ///
/* FP:lib.rs-1371 */ /// This is intended specifically to mirror LLVM’s `!range` metadata semantics.
/* FP:lib.rs-1372 */ #[derive(Clone, Copy, PartialEq, Eq, Hash)]
/* FP:lib.rs-1373 */ #[cfg_attr(feature = "nightly", derive(HashStable_Generic))]
/* FP:lib.rs-1374 */ pub struct WrappingRange {
/* FP:lib.rs-1375 */     pub start: u128,
/* FP:lib.rs-1376 */     pub end: u128,
/* FP:lib.rs-1377 */ }
/* FP:lib.rs-1378 */ 
/* FP:lib.rs-1379 */ impl WrappingRange {
/* FP:lib.rs-1380 */     pub fn full(size: Size) -> Self {
/* FP:lib.rs-1381 */         Self { start: 0, end: size.unsigned_int_max() }
/* FP:lib.rs-1382 */     }
/* FP:lib.rs-1383 */ 
/* FP:lib.rs-1384 */     /// Returns `true` if `v` is contained in the range.
/* FP:lib.rs-1385 */     #[inline(always)]
/* FP:lib.rs-1386 */     pub fn contains(&self, v: u128) -> bool {
/* FP:lib.rs-1387 */         if self.start <= self.end {
/* FP:lib.rs-1388 */             self.start <= v && v <= self.end
/* FP:lib.rs-1389 */         } else {
/* FP:lib.rs-1390 */             self.start <= v || v <= self.end
/* FP:lib.rs-1391 */         }
/* FP:lib.rs-1392 */     }
/* FP:lib.rs-1393 */ 
/* FP:lib.rs-1394 */     /// Returns `true` if all the values in `other` are contained in this range,
/* FP:lib.rs-1395 */     /// when the values are considered as having width `size`.
/* FP:lib.rs-1396 */     #[inline(always)]
/* FP:lib.rs-1397 */     pub fn contains_range(&self, other: Self, size: Size) -> bool {
/* FP:lib.rs-1398 */         if self.is_full_for(size) {
/* FP:lib.rs-1399 */             true
/* FP:lib.rs-1400 */         } else {
/* FP:lib.rs-1401 */             let trunc = |x| size.truncate(x);
/* FP:lib.rs-1402 */ 
/* FP:lib.rs-1403 */             let delta = self.start;
/* FP:lib.rs-1404 */             let max = trunc(self.end.wrapping_sub(delta));
/* FP:lib.rs-1405 */ 
/* FP:lib.rs-1406 */             let other_start = trunc(other.start.wrapping_sub(delta));
/* FP:lib.rs-1407 */             let other_end = trunc(other.end.wrapping_sub(delta));
/* FP:lib.rs-1408 */ 
/* FP:lib.rs-1409 */             // Having shifted both input ranges by `delta`, now we only need to check
/* FP:lib.rs-1410 */             // whether `0..=max` contains `other_start..=other_end`, which can only
/* FP:lib.rs-1411 */             // happen if the other doesn't wrap since `self` isn't everything.
/* FP:lib.rs-1412 */             (other_start <= other_end) && (other_end <= max)
/* FP:lib.rs-1413 */         }
/* FP:lib.rs-1414 */     }
/* FP:lib.rs-1415 */ 
/* FP:lib.rs-1416 */     /// Returns `self` with replaced `start`
/* FP:lib.rs-1417 */     #[inline(always)]
/* FP:lib.rs-1418 */     fn with_start(mut self, start: u128) -> Self {
/* FP:lib.rs-1419 */         self.start = start;
/* FP:lib.rs-1420 */         self
/* FP:lib.rs-1421 */     }
/* FP:lib.rs-1422 */ 
/* FP:lib.rs-1423 */     /// Returns `self` with replaced `end`
/* FP:lib.rs-1424 */     #[inline(always)]
/* FP:lib.rs-1425 */     fn with_end(mut self, end: u128) -> Self {
/* FP:lib.rs-1426 */         self.end = end;
/* FP:lib.rs-1427 */         self
/* FP:lib.rs-1428 */     }
/* FP:lib.rs-1429 */ 
/* FP:lib.rs-1430 */     /// Returns `true` if `size` completely fills the range.
/* FP:lib.rs-1431 */     ///
/* FP:lib.rs-1432 */     /// Note that this is *not* the same as `self == WrappingRange::full(size)`.
/* FP:lib.rs-1433 */     /// Niche calculations can produce full ranges which are not the canonical one;
/* FP:lib.rs-1434 */     /// for example `Option<NonZero<u16>>` gets `valid_range: (..=0) | (1..)`.
/* FP:lib.rs-1435 */     #[inline]
/* FP:lib.rs-1436 */     fn is_full_for(&self, size: Size) -> bool {
/* FP:lib.rs-1437 */         let max_value = size.unsigned_int_max();
/* FP:lib.rs-1438 */         debug_assert!(self.start <= max_value && self.end <= max_value);
/* FP:lib.rs-1439 */         self.start == (self.end.wrapping_add(1) & max_value)
/* FP:lib.rs-1440 */     }
/* FP:lib.rs-1441 */ 
/* FP:lib.rs-1442 */     /// Checks whether this range is considered non-wrapping when the values are
/* FP:lib.rs-1443 */     /// interpreted as *unsigned* numbers of width `size`.
/* FP:lib.rs-1444 */     ///
/* FP:lib.rs-1445 */     /// Returns `Ok(true)` if there's no wrap-around, `Ok(false)` if there is,
/* FP:lib.rs-1446 */     /// and `Err(..)` if the range is full so it depends how you think about it.
/* FP:lib.rs-1447 */     #[inline]
/* FP:lib.rs-1448 */     pub fn no_unsigned_wraparound(&self, size: Size) -> Result<bool, RangeFull> {
/* FP:lib.rs-1449 */         if self.is_full_for(size) { Err(..) } else { Ok(self.start <= self.end) }
/* FP:lib.rs-1450 */     }
/* FP:lib.rs-1451 */ 
/* FP:lib.rs-1452 */     /// Checks whether this range is considered non-wrapping when the values are
/* FP:lib.rs-1453 */     /// interpreted as *signed* numbers of width `size`.
/* FP:lib.rs-1454 */     ///
/* FP:lib.rs-1455 */     /// This is heavily dependent on the `size`, as `100..=200` does wrap when
/* FP:lib.rs-1456 */     /// interpreted as `i8`, but doesn't when interpreted as `i16`.
/* FP:lib.rs-1457 */     ///
/* FP:lib.rs-1458 */     /// Returns `Ok(true)` if there's no wrap-around, `Ok(false)` if there is,
/* FP:lib.rs-1459 */     /// and `Err(..)` if the range is full so it depends how you think about it.
/* FP:lib.rs-1460 */     #[inline]
/* FP:lib.rs-1461 */     pub fn no_signed_wraparound(&self, size: Size) -> Result<bool, RangeFull> {
/* FP:lib.rs-1462 */         if self.is_full_for(size) {
/* FP:lib.rs-1463 */             Err(..)
/* FP:lib.rs-1464 */         } else {
/* FP:lib.rs-1465 */             let start: i128 = size.sign_extend(self.start);
/* FP:lib.rs-1466 */             let end: i128 = size.sign_extend(self.end);
/* FP:lib.rs-1467 */             Ok(start <= end)
/* FP:lib.rs-1468 */         }
/* FP:lib.rs-1469 */     }
/* FP:lib.rs-1470 */ }
/* FP:lib.rs-1471 */ 
/* FP:lib.rs-1472 */ impl fmt::Debug for WrappingRange {
/* FP:lib.rs-1473 */     fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:lib.rs-1474 */         if self.start > self.end {
/* FP:lib.rs-1475 */             write!(fmt, "(..={}) | ({}..)", self.end, self.start)?;
/* FP:lib.rs-1476 */         } else {
/* FP:lib.rs-1477 */             write!(fmt, "{}..={}", self.start, self.end)?;
/* FP:lib.rs-1478 */         }
/* FP:lib.rs-1479 */         Ok(())
/* FP:lib.rs-1480 */     }
/* FP:lib.rs-1481 */ }
/* FP:lib.rs-1482 */ 
/* FP:lib.rs-1483 */ /// Information about one scalar component of a Rust type.
/* FP:lib.rs-1484 */ #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
/* FP:lib.rs-1485 */ #[cfg_attr(feature = "nightly", derive(HashStable_Generic))]
/* FP:lib.rs-1486 */ pub enum Scalar {
/* FP:lib.rs-1487 */     Initialized {
/* FP:lib.rs-1488 */         value: Primitive,
/* FP:lib.rs-1489 */ 
/* FP:lib.rs-1490 */         // FIXME(eddyb) always use the shortest range, e.g., by finding
/* FP:lib.rs-1491 */         // the largest space between two consecutive valid values and
/* FP:lib.rs-1492 */         // taking everything else as the (shortest) valid range.
/* FP:lib.rs-1493 */         valid_range: WrappingRange,
/* FP:lib.rs-1494 */     },
/* FP:lib.rs-1495 */     Union {
/* FP:lib.rs-1496 */         /// Even for unions, we need to use the correct registers for the kind of
/* FP:lib.rs-1497 */         /// values inside the union, so we keep the `Primitive` type around. We
/* FP:lib.rs-1498 */         /// also use it to compute the size of the scalar.
/* FP:lib.rs-1499 */         /// However, unions never have niches and even allow undef,
/* FP:lib.rs-1500 */         /// so there is no `valid_range`.
/* FP:lib.rs-1501 */         value: Primitive,
/* FP:lib.rs-1502 */     },
/* FP:lib.rs-1503 */ }
/* FP:lib.rs-1504 */ 
/* FP:lib.rs-1505 */ impl Scalar {
/* FP:lib.rs-1506 */     #[inline]
/* FP:lib.rs-1507 */     pub fn is_bool(&self) -> bool {
/* FP:lib.rs-1508 */         use Integer::*;
/* FP:lib.rs-1509 */         matches!(
/* FP:lib.rs-1510 */             self,
/* FP:lib.rs-1511 */             Scalar::Initialized {
/* FP:lib.rs-1512 */                 value: Primitive::Int(I8, false),
/* FP:lib.rs-1513 */                 valid_range: WrappingRange { start: 0, end: 1 }
/* FP:lib.rs-1514 */             }
/* FP:lib.rs-1515 */         )
/* FP:lib.rs-1516 */     }
/* FP:lib.rs-1517 */ 
/* FP:lib.rs-1518 */     /// Get the primitive representation of this type, ignoring the valid range and whether the
/* FP:lib.rs-1519 */     /// value is allowed to be undefined (due to being a union).
/* FP:lib.rs-1520 */     pub fn primitive(&self) -> Primitive {
/* FP:lib.rs-1521 */         match *self {
/* FP:lib.rs-1522 */             Scalar::Initialized { value, .. } | Scalar::Union { value } => value,
/* FP:lib.rs-1523 */         }
/* FP:lib.rs-1524 */     }
/* FP:lib.rs-1525 */ 
/* FP:lib.rs-1526 */     pub fn align(self, cx: &impl HasDataLayout) -> AbiAlign {
/* FP:lib.rs-1527 */         self.primitive().align(cx)
/* FP:lib.rs-1528 */     }
/* FP:lib.rs-1529 */ 
/* FP:lib.rs-1530 */     pub fn size(self, cx: &impl HasDataLayout) -> Size {
/* FP:lib.rs-1531 */         self.primitive().size(cx)
/* FP:lib.rs-1532 */     }
/* FP:lib.rs-1533 */ 
/* FP:lib.rs-1534 */     #[inline]
/* FP:lib.rs-1535 */     pub fn to_union(&self) -> Self {
/* FP:lib.rs-1536 */         Self::Union { value: self.primitive() }
/* FP:lib.rs-1537 */     }
/* FP:lib.rs-1538 */ 
/* FP:lib.rs-1539 */     #[inline]
/* FP:lib.rs-1540 */     pub fn valid_range(&self, cx: &impl HasDataLayout) -> WrappingRange {
/* FP:lib.rs-1541 */         match *self {
/* FP:lib.rs-1542 */             Scalar::Initialized { valid_range, .. } => valid_range,
/* FP:lib.rs-1543 */             Scalar::Union { value } => WrappingRange::full(value.size(cx)),
/* FP:lib.rs-1544 */         }
/* FP:lib.rs-1545 */     }
/* FP:lib.rs-1546 */ 
/* FP:lib.rs-1547 */     #[inline]
/* FP:lib.rs-1548 */     /// Allows the caller to mutate the valid range. This operation will panic if attempted on a
/* FP:lib.rs-1549 */     /// union.
/* FP:lib.rs-1550 */     pub fn valid_range_mut(&mut self) -> &mut WrappingRange {
/* FP:lib.rs-1551 */         match self {
/* FP:lib.rs-1552 */             Scalar::Initialized { valid_range, .. } => valid_range,
/* FP:lib.rs-1553 */             Scalar::Union { .. } => panic!("cannot change the valid range of a union"),
/* FP:lib.rs-1554 */         }
/* FP:lib.rs-1555 */     }
/* FP:lib.rs-1556 */ 
/* FP:lib.rs-1557 */     /// Returns `true` if all possible numbers are valid, i.e `valid_range` covers the whole
/* FP:lib.rs-1558 */     /// layout.
/* FP:lib.rs-1559 */     #[inline]
/* FP:lib.rs-1560 */     pub fn is_always_valid<C: HasDataLayout>(&self, cx: &C) -> bool {
/* FP:lib.rs-1561 */         match *self {
/* FP:lib.rs-1562 */             Scalar::Initialized { valid_range, .. } => valid_range.is_full_for(self.size(cx)),
/* FP:lib.rs-1563 */             Scalar::Union { .. } => true,
/* FP:lib.rs-1564 */         }
/* FP:lib.rs-1565 */     }
/* FP:lib.rs-1566 */ 
/* FP:lib.rs-1567 */     /// Returns `true` if this type can be left uninit.
/* FP:lib.rs-1568 */     #[inline]
/* FP:lib.rs-1569 */     pub fn is_uninit_valid(&self) -> bool {
/* FP:lib.rs-1570 */         match *self {
/* FP:lib.rs-1571 */             Scalar::Initialized { .. } => false,
/* FP:lib.rs-1572 */             Scalar::Union { .. } => true,
/* FP:lib.rs-1573 */         }
/* FP:lib.rs-1574 */     }
/* FP:lib.rs-1575 */ 
/* FP:lib.rs-1576 */     /// Returns `true` if this is a signed integer scalar
/* FP:lib.rs-1577 */     #[inline]
/* FP:lib.rs-1578 */     pub fn is_signed(&self) -> bool {
/* FP:lib.rs-1579 */         match self.primitive() {
/* FP:lib.rs-1580 */             Primitive::Int(_, signed) => signed,
/* FP:lib.rs-1581 */             _ => false,
/* FP:lib.rs-1582 */         }
/* FP:lib.rs-1583 */     }
/* FP:lib.rs-1584 */ }
/* FP:lib.rs-1585 */ 
/* FP:lib.rs-1586 */ // NOTE: This struct is generic over the FieldIdx for rust-analyzer usage.
/* FP:lib.rs-1587 */ /// Describes how the fields of a type are located in memory.
/* FP:lib.rs-1588 */ #[derive(PartialEq, Eq, Hash, Clone, Debug)]
/* FP:lib.rs-1589 */ #[cfg_attr(feature = "nightly", derive(HashStable_Generic))]
/* FP:lib.rs-1590 */ pub enum FieldsShape<FieldIdx: Idx> {
/* FP:lib.rs-1591 */     /// Scalar primitives and `!`, which never have fields.
/* FP:lib.rs-1592 */     Primitive,
/* FP:lib.rs-1593 */ 
/* FP:lib.rs-1594 */     /// All fields start at no offset. The `usize` is the field count.
/* FP:lib.rs-1595 */     Union(NonZeroUsize),
/* FP:lib.rs-1596 */ 
/* FP:lib.rs-1597 */     /// Array/vector-like placement, with all fields of identical types.
/* FP:lib.rs-1598 */     Array { stride: Size, count: u64 },
/* FP:lib.rs-1599 */ 
/* FP:lib.rs-1600 */     /// Struct-like placement, with precomputed offsets.
/* FP:lib.rs-1601 */     ///
/* FP:lib.rs-1602 */     /// Fields are guaranteed to not overlap, but note that gaps
/* FP:lib.rs-1603 */     /// before, between and after all the fields are NOT always
/* FP:lib.rs-1604 */     /// padding, and as such their contents may not be discarded.
/* FP:lib.rs-1605 */     /// For example, enum variants leave a gap at the start,
/* FP:lib.rs-1606 */     /// where the discriminant field in the enum layout goes.
/* FP:lib.rs-1607 */     Arbitrary {
/* FP:lib.rs-1608 */         /// Offsets for the first byte of each field,
/* FP:lib.rs-1609 */         /// ordered to match the source definition order.
/* FP:lib.rs-1610 */         /// This vector does not go in increasing order.
/* FP:lib.rs-1611 */         // FIXME(eddyb) use small vector optimization for the common case.
/* FP:lib.rs-1612 */         offsets: IndexVec<FieldIdx, Size>,
/* FP:lib.rs-1613 */ 
/* FP:lib.rs-1614 */         /// Maps source order field indices to memory order indices,
/* FP:lib.rs-1615 */         /// depending on how the fields were reordered (if at all).
/* FP:lib.rs-1616 */         /// This is a permutation, with both the source order and the
/* FP:lib.rs-1617 */         /// memory order using the same (0..n) index ranges.
/* FP:lib.rs-1618 */         ///
/* FP:lib.rs-1619 */         /// Note that during computation of `memory_index`, sometimes
/* FP:lib.rs-1620 */         /// it is easier to operate on the inverse mapping (that is,
/* FP:lib.rs-1621 */         /// from memory order to source order), and that is usually
/* FP:lib.rs-1622 */         /// named `inverse_memory_index`.
/* FP:lib.rs-1623 */         ///
/* FP:lib.rs-1624 */         // FIXME(eddyb) build a better abstraction for permutations, if possible.
/* FP:lib.rs-1625 */         // FIXME(camlorn) also consider small vector optimization here.
/* FP:lib.rs-1626 */         memory_index: IndexVec<FieldIdx, u32>,
/* FP:lib.rs-1627 */     },
/* FP:lib.rs-1628 */ }
/* FP:lib.rs-1629 */ 
/* FP:lib.rs-1630 */ impl<FieldIdx: Idx> FieldsShape<FieldIdx> {
/* FP:lib.rs-1631 */     #[inline]
/* FP:lib.rs-1632 */     pub fn count(&self) -> usize {
/* FP:lib.rs-1633 */         match *self {
/* FP:lib.rs-1634 */             FieldsShape::Primitive => 0,
/* FP:lib.rs-1635 */             FieldsShape::Union(count) => count.get(),
/* FP:lib.rs-1636 */             FieldsShape::Array { count, .. } => count.try_into().unwrap(),
/* FP:lib.rs-1637 */             FieldsShape::Arbitrary { ref offsets, .. } => offsets.len(),
/* FP:lib.rs-1638 */         }
/* FP:lib.rs-1639 */     }
/* FP:lib.rs-1640 */ 
/* FP:lib.rs-1641 */     #[inline]
/* FP:lib.rs-1642 */     pub fn offset(&self, i: usize) -> Size {
/* FP:lib.rs-1643 */         match *self {
/* FP:lib.rs-1644 */             FieldsShape::Primitive => {
/* FP:lib.rs-1645 */                 unreachable!("FieldsShape::offset: `Primitive`s have no fields")
/* FP:lib.rs-1646 */             }
/* FP:lib.rs-1647 */             FieldsShape::Union(count) => {
/* FP:lib.rs-1648 */                 assert!(i < count.get(), "tried to access field {i} of union with {count} fields");
/* FP:lib.rs-1649 */                 Size::ZERO
/* FP:lib.rs-1650 */             }
/* FP:lib.rs-1651 */             FieldsShape::Array { stride, count } => {
/* FP:lib.rs-1652 */                 let i = u64::try_from(i).unwrap();
/* FP:lib.rs-1653 */                 assert!(i < count, "tried to access field {i} of array with {count} fields");
/* FP:lib.rs-1654 */                 stride * i
/* FP:lib.rs-1655 */             }
/* FP:lib.rs-1656 */             FieldsShape::Arbitrary { ref offsets, .. } => offsets[FieldIdx::new(i)],
/* FP:lib.rs-1657 */         }
/* FP:lib.rs-1658 */     }
/* FP:lib.rs-1659 */ 
/* FP:lib.rs-1660 */     #[inline]
/* FP:lib.rs-1661 */     pub fn memory_index(&self, i: usize) -> usize {
/* FP:lib.rs-1662 */         match *self {
/* FP:lib.rs-1663 */             FieldsShape::Primitive => {
/* FP:lib.rs-1664 */                 unreachable!("FieldsShape::memory_index: `Primitive`s have no fields")
/* FP:lib.rs-1665 */             }
/* FP:lib.rs-1666 */             FieldsShape::Union(_) | FieldsShape::Array { .. } => i,
/* FP:lib.rs-1667 */             FieldsShape::Arbitrary { ref memory_index, .. } => {
/* FP:lib.rs-1668 */                 memory_index[FieldIdx::new(i)].try_into().unwrap()
/* FP:lib.rs-1669 */             }
/* FP:lib.rs-1670 */         }
/* FP:lib.rs-1671 */     }
/* FP:lib.rs-1672 */ 
/* FP:lib.rs-1673 */     /// Gets source indices of the fields by increasing offsets.
/* FP:lib.rs-1674 */     #[inline]
/* FP:lib.rs-1675 */     pub fn index_by_increasing_offset(&self) -> impl ExactSizeIterator<Item = usize> {
/* FP:lib.rs-1676 */         let mut inverse_small = [0u8; 64];
/* FP:lib.rs-1677 */         let mut inverse_big = IndexVec::new();
/* FP:lib.rs-1678 */         let use_small = self.count() <= inverse_small.len();
/* FP:lib.rs-1679 */ 
/* FP:lib.rs-1680 */         // We have to write this logic twice in order to keep the array small.
/* FP:lib.rs-1681 */         if let FieldsShape::Arbitrary { ref memory_index, .. } = *self {
/* FP:lib.rs-1682 */             if use_small {
/* FP:lib.rs-1683 */                 for (field_idx, &mem_idx) in memory_index.iter_enumerated() {
/* FP:lib.rs-1684 */                     inverse_small[mem_idx as usize] = field_idx.index() as u8;
/* FP:lib.rs-1685 */                 }
/* FP:lib.rs-1686 */             } else {
/* FP:lib.rs-1687 */                 inverse_big = memory_index.invert_bijective_mapping();
/* FP:lib.rs-1688 */             }
/* FP:lib.rs-1689 */         }
/* FP:lib.rs-1690 */ 
/* FP:lib.rs-1691 */         // Primitives don't really have fields in the way that structs do,
/* FP:lib.rs-1692 */         // but having this return an empty iterator for them is unhelpful
/* FP:lib.rs-1693 */         // since that makes them look kinda like ZSTs, which they're not.
/* FP:lib.rs-1694 */         let pseudofield_count = if let FieldsShape::Primitive = self { 1 } else { self.count() };
/* FP:lib.rs-1695 */ 
/* FP:lib.rs-1696 */         (0..pseudofield_count).map(move |i| match *self {
/* FP:lib.rs-1697 */             FieldsShape::Primitive | FieldsShape::Union(_) | FieldsShape::Array { .. } => i,
/* FP:lib.rs-1698 */             FieldsShape::Arbitrary { .. } => {
/* FP:lib.rs-1699 */                 if use_small {
/* FP:lib.rs-1700 */                     inverse_small[i] as usize
/* FP:lib.rs-1701 */                 } else {
/* FP:lib.rs-1702 */                     inverse_big[i as u32].index()
/* FP:lib.rs-1703 */                 }
/* FP:lib.rs-1704 */             }
/* FP:lib.rs-1705 */         })
/* FP:lib.rs-1706 */     }
/* FP:lib.rs-1707 */ }
/* FP:lib.rs-1708 */ 
/* FP:lib.rs-1709 */ /// An identifier that specifies the address space that some operation
/* FP:lib.rs-1710 */ /// should operate on. Special address spaces have an effect on code generation,
/* FP:lib.rs-1711 */ /// depending on the target and the address spaces it implements.
/* FP:lib.rs-1712 */ #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/* FP:lib.rs-1713 */ #[cfg_attr(feature = "nightly", derive(HashStable_Generic))]
/* FP:lib.rs-1714 */ pub struct AddressSpace(pub u32);
/* FP:lib.rs-1715 */ 
/* FP:lib.rs-1716 */ impl AddressSpace {
/* FP:lib.rs-1717 */     /// LLVM's `0` address space.
/* FP:lib.rs-1718 */     pub const ZERO: Self = AddressSpace(0);
/* FP:lib.rs-1719 */ }
/* FP:lib.rs-1720 */ 
/* FP:lib.rs-1721 */ /// The way we represent values to the backend
/* FP:lib.rs-1722 */ ///
/* FP:lib.rs-1723 */ /// Previously this was conflated with the "ABI" a type is given, as in the platform-specific ABI.
/* FP:lib.rs-1724 */ /// In reality, this implies little about that, but is mostly used to describe the syntactic form
/* FP:lib.rs-1725 */ /// emitted for the backend, as most backends handle SSA values and blobs of memory differently.
/* FP:lib.rs-1726 */ /// The psABI may need consideration in doing so, but this enum does not constitute a promise for
/* FP:lib.rs-1727 */ /// how the value will be lowered to the calling convention, in itself.
/* FP:lib.rs-1728 */ ///
/* FP:lib.rs-1729 */ /// Generally, a codegen backend will prefer to handle smaller values as a scalar or short vector,
/* FP:lib.rs-1730 */ /// and larger values will usually prefer to be represented as memory.
/* FP:lib.rs-1731 */ #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
/* FP:lib.rs-1732 */ #[cfg_attr(feature = "nightly", derive(HashStable_Generic))]
/* FP:lib.rs-1733 */ pub enum BackendRepr {
/* FP:lib.rs-1734 */     Scalar(Scalar),
/* FP:lib.rs-1735 */     ScalarPair(Scalar, Scalar),
/* FP:lib.rs-1736 */     SimdVector {
/* FP:lib.rs-1737 */         element: Scalar,
/* FP:lib.rs-1738 */         count: u64,
/* FP:lib.rs-1739 */     },
/* FP:lib.rs-1740 */     // FIXME: I sometimes use memory, sometimes use an IR aggregate!
/* FP:lib.rs-1741 */     Memory {
/* FP:lib.rs-1742 */         /// If true, the size is exact, otherwise it's only a lower bound.
/* FP:lib.rs-1743 */         sized: bool,
/* FP:lib.rs-1744 */     },
/* FP:lib.rs-1745 */ }
/* FP:lib.rs-1746 */ 
/* FP:lib.rs-1747 */ impl BackendRepr {
/* FP:lib.rs-1748 */     /// Returns `true` if the layout corresponds to an unsized type.
/* FP:lib.rs-1749 */     #[inline]
/* FP:lib.rs-1750 */     pub fn is_unsized(&self) -> bool {
/* FP:lib.rs-1751 */         match *self {
/* FP:lib.rs-1752 */             BackendRepr::Scalar(_)
/* FP:lib.rs-1753 */             | BackendRepr::ScalarPair(..)
/* FP:lib.rs-1754 */             | BackendRepr::SimdVector { .. } => false,
/* FP:lib.rs-1755 */             BackendRepr::Memory { sized } => !sized,
/* FP:lib.rs-1756 */         }
/* FP:lib.rs-1757 */     }
/* FP:lib.rs-1758 */ 
/* FP:lib.rs-1759 */     #[inline]
/* FP:lib.rs-1760 */     pub fn is_sized(&self) -> bool {
/* FP:lib.rs-1761 */         !self.is_unsized()
/* FP:lib.rs-1762 */     }
/* FP:lib.rs-1763 */ 
/* FP:lib.rs-1764 */     /// Returns `true` if this is a single signed integer scalar.
/* FP:lib.rs-1765 */     /// Sanity check: panics if this is not a scalar type (see PR #70189).
/* FP:lib.rs-1766 */     #[inline]
/* FP:lib.rs-1767 */     pub fn is_signed(&self) -> bool {
/* FP:lib.rs-1768 */         match self {
/* FP:lib.rs-1769 */             BackendRepr::Scalar(scal) => scal.is_signed(),
/* FP:lib.rs-1770 */             _ => panic!("`is_signed` on non-scalar ABI {self:?}"),
/* FP:lib.rs-1771 */         }
/* FP:lib.rs-1772 */     }
/* FP:lib.rs-1773 */ 
/* FP:lib.rs-1774 */     /// Returns `true` if this is a scalar type
/* FP:lib.rs-1775 */     #[inline]
/* FP:lib.rs-1776 */     pub fn is_scalar(&self) -> bool {
/* FP:lib.rs-1777 */         matches!(*self, BackendRepr::Scalar(_))
/* FP:lib.rs-1778 */     }
/* FP:lib.rs-1779 */ 
/* FP:lib.rs-1780 */     /// Returns `true` if this is a bool
/* FP:lib.rs-1781 */     #[inline]
/* FP:lib.rs-1782 */     pub fn is_bool(&self) -> bool {
/* FP:lib.rs-1783 */         matches!(*self, BackendRepr::Scalar(s) if s.is_bool())
/* FP:lib.rs-1784 */     }
/* FP:lib.rs-1785 */ 
/* FP:lib.rs-1786 */     /// The psABI alignment for a `Scalar` or `ScalarPair`
/* FP:lib.rs-1787 */     ///
/* FP:lib.rs-1788 */     /// `None` for other variants.
/* FP:lib.rs-1789 */     pub fn scalar_align<C: HasDataLayout>(&self, cx: &C) -> Option<Align> {
/* FP:lib.rs-1790 */         match *self {
/* FP:lib.rs-1791 */             BackendRepr::Scalar(s) => Some(s.align(cx).abi),
/* FP:lib.rs-1792 */             BackendRepr::ScalarPair(s1, s2) => Some(s1.align(cx).max(s2.align(cx)).abi),
/* FP:lib.rs-1793 */             // The align of a Vector can vary in surprising ways
/* FP:lib.rs-1794 */             BackendRepr::SimdVector { .. } | BackendRepr::Memory { .. } => None,
/* FP:lib.rs-1795 */         }
/* FP:lib.rs-1796 */     }
/* FP:lib.rs-1797 */ 
/* FP:lib.rs-1798 */     /// The psABI size for a `Scalar` or `ScalarPair`
/* FP:lib.rs-1799 */     ///
/* FP:lib.rs-1800 */     /// `None` for other variants
/* FP:lib.rs-1801 */     pub fn scalar_size<C: HasDataLayout>(&self, cx: &C) -> Option<Size> {
/* FP:lib.rs-1802 */         match *self {
/* FP:lib.rs-1803 */             // No padding in scalars.
/* FP:lib.rs-1804 */             BackendRepr::Scalar(s) => Some(s.size(cx)),
/* FP:lib.rs-1805 */             // May have some padding between the pair.
/* FP:lib.rs-1806 */             BackendRepr::ScalarPair(s1, s2) => {
/* FP:lib.rs-1807 */                 let field2_offset = s1.size(cx).align_to(s2.align(cx).abi);
/* FP:lib.rs-1808 */                 let size = (field2_offset + s2.size(cx)).align_to(
/* FP:lib.rs-1809 */                     self.scalar_align(cx)
/* FP:lib.rs-1810 */                         // We absolutely must have an answer here or everything is FUBAR.
/* FP:lib.rs-1811 */                         .unwrap(),
/* FP:lib.rs-1812 */                 );
/* FP:lib.rs-1813 */                 Some(size)
/* FP:lib.rs-1814 */             }
/* FP:lib.rs-1815 */             // The size of a Vector can vary in surprising ways
/* FP:lib.rs-1816 */             BackendRepr::SimdVector { .. } | BackendRepr::Memory { .. } => None,
/* FP:lib.rs-1817 */         }
/* FP:lib.rs-1818 */     }
/* FP:lib.rs-1819 */ 
/* FP:lib.rs-1820 */     /// Discard validity range information and allow undef.
/* FP:lib.rs-1821 */     pub fn to_union(&self) -> Self {
/* FP:lib.rs-1822 */         match *self {
/* FP:lib.rs-1823 */             BackendRepr::Scalar(s) => BackendRepr::Scalar(s.to_union()),
/* FP:lib.rs-1824 */             BackendRepr::ScalarPair(s1, s2) => {
/* FP:lib.rs-1825 */                 BackendRepr::ScalarPair(s1.to_union(), s2.to_union())
/* FP:lib.rs-1826 */             }
/* FP:lib.rs-1827 */             BackendRepr::SimdVector { element, count } => {
/* FP:lib.rs-1828 */                 BackendRepr::SimdVector { element: element.to_union(), count }
/* FP:lib.rs-1829 */             }
/* FP:lib.rs-1830 */             BackendRepr::Memory { .. } => BackendRepr::Memory { sized: true },
/* FP:lib.rs-1831 */         }
/* FP:lib.rs-1832 */     }
/* FP:lib.rs-1833 */ 
/* FP:lib.rs-1834 */     pub fn eq_up_to_validity(&self, other: &Self) -> bool {
/* FP:lib.rs-1835 */         match (self, other) {
/* FP:lib.rs-1836 */             // Scalar, Vector, ScalarPair have `Scalar` in them where we ignore validity ranges.
/* FP:lib.rs-1837 */             // We do *not* ignore the sign since it matters for some ABIs (e.g. s390x).
/* FP:lib.rs-1838 */             (BackendRepr::Scalar(l), BackendRepr::Scalar(r)) => l.primitive() == r.primitive(),
/* FP:lib.rs-1839 */             (
/* FP:lib.rs-1840 */                 BackendRepr::SimdVector { element: element_l, count: count_l },
/* FP:lib.rs-1841 */                 BackendRepr::SimdVector { element: element_r, count: count_r },
/* FP:lib.rs-1842 */             ) => element_l.primitive() == element_r.primitive() && count_l == count_r,
/* FP:lib.rs-1843 */             (BackendRepr::ScalarPair(l1, l2), BackendRepr::ScalarPair(r1, r2)) => {
/* FP:lib.rs-1844 */                 l1.primitive() == r1.primitive() && l2.primitive() == r2.primitive()
/* FP:lib.rs-1845 */             }
/* FP:lib.rs-1846 */             // Everything else must be strictly identical.
/* FP:lib.rs-1847 */             _ => self == other,
/* FP:lib.rs-1848 */         }
/* FP:lib.rs-1849 */     }
/* FP:lib.rs-1850 */ }
/* FP:lib.rs-1851 */ 
/* FP:lib.rs-1852 */ // NOTE: This struct is generic over the FieldIdx and VariantIdx for rust-analyzer usage.
/* FP:lib.rs-1853 */ #[derive(PartialEq, Eq, Hash, Clone, Debug)]
/* FP:lib.rs-1854 */ #[cfg_attr(feature = "nightly", derive(HashStable_Generic))]
/* FP:lib.rs-1855 */ pub enum Variants<FieldIdx: Idx, VariantIdx: Idx> {
/* FP:lib.rs-1856 */     /// A type with no valid variants. Must be uninhabited.
/* FP:lib.rs-1857 */     Empty,
/* FP:lib.rs-1858 */ 
/* FP:lib.rs-1859 */     /// Single enum variants, structs/tuples, unions, and all non-ADTs.
/* FP:lib.rs-1860 */     Single {
/* FP:lib.rs-1861 */         /// Always `0` for types that cannot have multiple variants.
/* FP:lib.rs-1862 */         index: VariantIdx,
/* FP:lib.rs-1863 */     },
/* FP:lib.rs-1864 */ 
/* FP:lib.rs-1865 */     /// Enum-likes with more than one variant: each variant comes with
/* FP:lib.rs-1866 */     /// a *discriminant* (usually the same as the variant index but the user can
/* FP:lib.rs-1867 */     /// assign explicit discriminant values). That discriminant is encoded
/* FP:lib.rs-1868 */     /// as a *tag* on the machine. The layout of each variant is
/* FP:lib.rs-1869 */     /// a struct, and they all have space reserved for the tag.
/* FP:lib.rs-1870 */     /// For enums, the tag is the sole field of the layout.
/* FP:lib.rs-1871 */     Multiple {
/* FP:lib.rs-1872 */         tag: Scalar,
/* FP:lib.rs-1873 */         tag_encoding: TagEncoding<VariantIdx>,
/* FP:lib.rs-1874 */         tag_field: FieldIdx,
/* FP:lib.rs-1875 */         variants: IndexVec<VariantIdx, LayoutData<FieldIdx, VariantIdx>>,
/* FP:lib.rs-1876 */     },
/* FP:lib.rs-1877 */ }
/* FP:lib.rs-1878 */ 
/* FP:lib.rs-1879 */ // NOTE: This struct is generic over the VariantIdx for rust-analyzer usage.
/* FP:lib.rs-1880 */ #[derive(PartialEq, Eq, Hash, Clone, Debug)]
/* FP:lib.rs-1881 */ #[cfg_attr(feature = "nightly", derive(HashStable_Generic))]
/* FP:lib.rs-1882 */ pub enum TagEncoding<VariantIdx: Idx> {
/* FP:lib.rs-1883 */     /// The tag directly stores the discriminant, but possibly with a smaller layout
/* FP:lib.rs-1884 */     /// (so converting the tag to the discriminant can require sign extension).
/* FP:lib.rs-1885 */     Direct,
/* FP:lib.rs-1886 */ 
/* FP:lib.rs-1887 */     /// Niche (values invalid for a type) encoding the discriminant.
/* FP:lib.rs-1888 */     /// Note that for this encoding, the discriminant and variant index of each variant coincide!
/* FP:lib.rs-1889 */     /// This invariant is codified as part of [`layout_sanity_check`](../rustc_ty_utils/layout/invariant/fn.layout_sanity_check.html).
/* FP:lib.rs-1890 */     ///
/* FP:lib.rs-1891 */     /// The variant `untagged_variant` contains a niche at an arbitrary
/* FP:lib.rs-1892 */     /// offset (field [`Variants::Multiple::tag_field`] of the enum).
/* FP:lib.rs-1893 */     /// For a variant with variant index `i`, such that `i != untagged_variant`,
/* FP:lib.rs-1894 */     /// the tag is set to `(i - niche_variants.start).wrapping_add(niche_start)`
/* FP:lib.rs-1895 */     /// (this is wrapping arithmetic using the type of the niche field, cf. the
/* FP:lib.rs-1896 */     /// [`tag_for_variant`](../rustc_const_eval/interpret/struct.InterpCx.html#method.tag_for_variant)
/* FP:lib.rs-1897 */     /// query implementation).
/* FP:lib.rs-1898 */     /// To recover the variant index `i` from a `tag`, the above formula has to be reversed,
/* FP:lib.rs-1899 */     /// i.e. `i = tag.wrapping_sub(niche_start) + niche_variants.start`. If `i` ends up outside
/* FP:lib.rs-1900 */     /// `niche_variants`, the tag must have encoded the `untagged_variant`.
/* FP:lib.rs-1901 */     ///
/* FP:lib.rs-1902 */     /// For example, `Option<(usize, &T)>`  is represented such that the tag for
/* FP:lib.rs-1903 */     /// `None` is the null pointer in the second tuple field, and
/* FP:lib.rs-1904 */     /// `Some` is the identity function (with a non-null reference)
/* FP:lib.rs-1905 */     /// and has no additional tag, i.e. the reference being non-null uniquely identifies this variant.
/* FP:lib.rs-1906 */     ///
/* FP:lib.rs-1907 */     /// Other variants that are not `untagged_variant` and that are outside the `niche_variants`
/* FP:lib.rs-1908 */     /// range cannot be represented; they must be uninhabited.
/* FP:lib.rs-1909 */     /// Nonetheless, uninhabited variants can also fall into the range of `niche_variants`.
/* FP:lib.rs-1910 */     Niche {
/* FP:lib.rs-1911 */         untagged_variant: VariantIdx,
/* FP:lib.rs-1912 */         /// This range *may* contain `untagged_variant` or uninhabited variants;
/* FP:lib.rs-1913 */         /// these are then just "dead values" and not used to encode anything.
/* FP:lib.rs-1914 */         niche_variants: RangeInclusive<VariantIdx>,
/* FP:lib.rs-1915 */         /// This is inbounds of the type of the niche field
/* FP:lib.rs-1916 */         /// (not sign-extended, i.e., all bits beyond the niche field size are 0).
/* FP:lib.rs-1917 */         niche_start: u128,
/* FP:lib.rs-1918 */     },
/* FP:lib.rs-1919 */ }
/* FP:lib.rs-1920 */ 
/* FP:lib.rs-1921 */ #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
/* FP:lib.rs-1922 */ #[cfg_attr(feature = "nightly", derive(HashStable_Generic))]
/* FP:lib.rs-1923 */ pub struct Niche {
/* FP:lib.rs-1924 */     pub offset: Size,
/* FP:lib.rs-1925 */     pub value: Primitive,
/* FP:lib.rs-1926 */     pub valid_range: WrappingRange,
/* FP:lib.rs-1927 */ }
/* FP:lib.rs-1928 */ 
/* FP:lib.rs-1929 */ impl Niche {
/* FP:lib.rs-1930 */     pub fn from_scalar<C: HasDataLayout>(cx: &C, offset: Size, scalar: Scalar) -> Option<Self> {
/* FP:lib.rs-1931 */         let Scalar::Initialized { value, valid_range } = scalar else { return None };
/* FP:lib.rs-1932 */         let niche = Niche { offset, value, valid_range };
/* FP:lib.rs-1933 */         if niche.available(cx) > 0 { Some(niche) } else { None }
/* FP:lib.rs-1934 */     }
/* FP:lib.rs-1935 */ 
/* FP:lib.rs-1936 */     pub fn available<C: HasDataLayout>(&self, cx: &C) -> u128 {
/* FP:lib.rs-1937 */         let Self { value, valid_range: v, .. } = *self;
/* FP:lib.rs-1938 */         let size = value.size(cx);
/* FP:lib.rs-1939 */         assert!(size.bits() <= 128);
/* FP:lib.rs-1940 */         let max_value = size.unsigned_int_max();
/* FP:lib.rs-1941 */ 
/* FP:lib.rs-1942 */         // Find out how many values are outside the valid range.
/* FP:lib.rs-1943 */         let niche = v.end.wrapping_add(1)..v.start;
/* FP:lib.rs-1944 */         niche.end.wrapping_sub(niche.start) & max_value
/* FP:lib.rs-1945 */     }
/* FP:lib.rs-1946 */ 
/* FP:lib.rs-1947 */     pub fn reserve<C: HasDataLayout>(&self, cx: &C, count: u128) -> Option<(u128, Scalar)> {
/* FP:lib.rs-1948 */         assert!(count > 0);
/* FP:lib.rs-1949 */ 
/* FP:lib.rs-1950 */         let Self { value, valid_range: v, .. } = *self;
/* FP:lib.rs-1951 */         let size = value.size(cx);
/* FP:lib.rs-1952 */         assert!(size.bits() <= 128);
/* FP:lib.rs-1953 */         let max_value = size.unsigned_int_max();
/* FP:lib.rs-1954 */ 
/* FP:lib.rs-1955 */         let niche = v.end.wrapping_add(1)..v.start;
/* FP:lib.rs-1956 */         let available = niche.end.wrapping_sub(niche.start) & max_value;
/* FP:lib.rs-1957 */         if count > available {
/* FP:lib.rs-1958 */             return None;
/* FP:lib.rs-1959 */         }
/* FP:lib.rs-1960 */ 
/* FP:lib.rs-1961 */         // Extend the range of valid values being reserved by moving either `v.start` or `v.end`
/* FP:lib.rs-1962 */         // bound. Given an eventual `Option<T>`, we try to maximize the chance for `None` to occupy
/* FP:lib.rs-1963 */         // the niche of zero. This is accomplished by preferring enums with 2 variants(`count==1`)
/* FP:lib.rs-1964 */         // and always taking the shortest path to niche zero. Having `None` in niche zero can
/* FP:lib.rs-1965 */         // enable some special optimizations.
/* FP:lib.rs-1966 */         //
/* FP:lib.rs-1967 */         // Bound selection criteria:
/* FP:lib.rs-1968 */         // 1. Select closest to zero given wrapping semantics.
/* FP:lib.rs-1969 */         // 2. Avoid moving past zero if possible.
/* FP:lib.rs-1970 */         //
/* FP:lib.rs-1971 */         // In practice this means that enums with `count > 1` are unlikely to claim niche zero,
/* FP:lib.rs-1972 */         // since they have to fit perfectly. If niche zero is already reserved, the selection of
/* FP:lib.rs-1973 */         // bounds are of little interest.
/* FP:lib.rs-1974 */         let move_start = |v: WrappingRange| {
/* FP:lib.rs-1975 */             let start = v.start.wrapping_sub(count) & max_value;
/* FP:lib.rs-1976 */             Some((start, Scalar::Initialized { value, valid_range: v.with_start(start) }))
/* FP:lib.rs-1977 */         };
/* FP:lib.rs-1978 */         let move_end = |v: WrappingRange| {
/* FP:lib.rs-1979 */             let start = v.end.wrapping_add(1) & max_value;
/* FP:lib.rs-1980 */             let end = v.end.wrapping_add(count) & max_value;
/* FP:lib.rs-1981 */             Some((start, Scalar::Initialized { value, valid_range: v.with_end(end) }))
/* FP:lib.rs-1982 */         };
/* FP:lib.rs-1983 */         let distance_end_zero = max_value - v.end;
/* FP:lib.rs-1984 */         if v.start > v.end {
/* FP:lib.rs-1985 */             // zero is unavailable because wrapping occurs
/* FP:lib.rs-1986 */             move_end(v)
/* FP:lib.rs-1987 */         } else if v.start <= distance_end_zero {
/* FP:lib.rs-1988 */             if count <= v.start {
/* FP:lib.rs-1989 */                 move_start(v)
/* FP:lib.rs-1990 */             } else {
/* FP:lib.rs-1991 */                 // moved past zero, use other bound
/* FP:lib.rs-1992 */                 move_end(v)
/* FP:lib.rs-1993 */             }
/* FP:lib.rs-1994 */         } else {
/* FP:lib.rs-1995 */             let end = v.end.wrapping_add(count) & max_value;
/* FP:lib.rs-1996 */             let overshot_zero = (1..=v.end).contains(&end);
/* FP:lib.rs-1997 */             if overshot_zero {
/* FP:lib.rs-1998 */                 // moved past zero, use other bound
/* FP:lib.rs-1999 */                 move_start(v)
/* FP:lib.rs-2000 */             } else {
/* FP:lib.rs-2001 */                 move_end(v)
/* FP:lib.rs-2002 */             }
/* FP:lib.rs-2003 */         }
/* FP:lib.rs-2004 */     }
/* FP:lib.rs-2005 */ }
/* FP:lib.rs-2006 */ 
/* FP:lib.rs-2007 */ // NOTE: This struct is generic over the FieldIdx and VariantIdx for rust-analyzer usage.
/* FP:lib.rs-2008 */ #[derive(PartialEq, Eq, Hash, Clone)]
/* FP:lib.rs-2009 */ #[cfg_attr(feature = "nightly", derive(HashStable_Generic))]
/* FP:lib.rs-2010 */ pub struct LayoutData<FieldIdx: Idx, VariantIdx: Idx> {
/* FP:lib.rs-2011 */     /// Says where the fields are located within the layout.
/* FP:lib.rs-2012 */     pub fields: FieldsShape<FieldIdx>,
/* FP:lib.rs-2013 */ 
/* FP:lib.rs-2014 */     /// Encodes information about multi-variant layouts.
/* FP:lib.rs-2015 */     /// Even with `Multiple` variants, a layout still has its own fields! Those are then
/* FP:lib.rs-2016 */     /// shared between all variants. One of them will be the discriminant,
/* FP:lib.rs-2017 */     /// but e.g. coroutines can have more.
/* FP:lib.rs-2018 */     ///
/* FP:lib.rs-2019 */     /// To access all fields of this layout, both `fields` and the fields of the active variant
/* FP:lib.rs-2020 */     /// must be taken into account.
/* FP:lib.rs-2021 */     pub variants: Variants<FieldIdx, VariantIdx>,
/* FP:lib.rs-2022 */ 
/* FP:lib.rs-2023 */     /// The `backend_repr` defines how this data will be represented to the codegen backend,
/* FP:lib.rs-2024 */     /// and encodes value restrictions via `valid_range`.
/* FP:lib.rs-2025 */     ///
/* FP:lib.rs-2026 */     /// Note that this is entirely orthogonal to the recursive structure defined by
/* FP:lib.rs-2027 */     /// `variants` and `fields`; for example, `ManuallyDrop<Result<isize, isize>>` has
/* FP:lib.rs-2028 */     /// `IrForm::ScalarPair`! So, even with non-`Memory` `backend_repr`, `fields` and `variants`
/* FP:lib.rs-2029 */     /// have to be taken into account to find all fields of this layout.
/* FP:lib.rs-2030 */     pub backend_repr: BackendRepr,
/* FP:lib.rs-2031 */ 
/* FP:lib.rs-2032 */     /// The leaf scalar with the largest number of invalid values
/* FP:lib.rs-2033 */     /// (i.e. outside of its `valid_range`), if it exists.
/* FP:lib.rs-2034 */     pub largest_niche: Option<Niche>,
/* FP:lib.rs-2035 */     /// Is this type known to be uninhabted?
/* FP:lib.rs-2036 */     ///
/* FP:lib.rs-2037 */     /// This is separate from BackendRepr because uninhabited return types can affect ABI,
/* FP:lib.rs-2038 */     /// especially in the case of by-pointer struct returns, which allocate stack even when unused.
/* FP:lib.rs-2039 */     pub uninhabited: bool,
/* FP:lib.rs-2040 */ 
/* FP:lib.rs-2041 */     pub align: AbiAlign,
/* FP:lib.rs-2042 */     pub size: Size,
/* FP:lib.rs-2043 */ 
/* FP:lib.rs-2044 */     /// The largest alignment explicitly requested with `repr(align)` on this type or any field.
/* FP:lib.rs-2045 */     /// Only used on i686-windows, where the argument passing ABI is different when alignment is
/* FP:lib.rs-2046 */     /// requested, even if the requested alignment is equal to the natural alignment.
/* FP:lib.rs-2047 */     pub max_repr_align: Option<Align>,
/* FP:lib.rs-2048 */ 
/* FP:lib.rs-2049 */     /// The alignment the type would have, ignoring any `repr(align)` but including `repr(packed)`.
/* FP:lib.rs-2050 */     /// Only used on aarch64-linux, where the argument passing ABI ignores the requested alignment
/* FP:lib.rs-2051 */     /// in some cases.
/* FP:lib.rs-2052 */     pub unadjusted_abi_align: Align,
/* FP:lib.rs-2053 */ 
/* FP:lib.rs-2054 */     /// The randomization seed based on this type's own repr and its fields.
/* FP:lib.rs-2055 */     ///
/* FP:lib.rs-2056 */     /// Since randomization is toggled on a per-crate basis even crates that do not have randomization
/* FP:lib.rs-2057 */     /// enabled should still calculate a seed so that downstream uses can use it to distinguish different
/* FP:lib.rs-2058 */     /// types.
/* FP:lib.rs-2059 */     ///
/* FP:lib.rs-2060 */     /// For every T and U for which we do not guarantee that a repr(Rust) `Foo<T>` can be coerced or
/* FP:lib.rs-2061 */     /// transmuted to `Foo<U>` we aim to create probalistically distinct seeds so that Foo can choose
/* FP:lib.rs-2062 */     /// to reorder its fields based on that information. The current implementation is a conservative
/* FP:lib.rs-2063 */     /// approximation of this goal.
/* FP:lib.rs-2064 */     pub randomization_seed: Hash64,
/* FP:lib.rs-2065 */ }
/* FP:lib.rs-2066 */ 
/* FP:lib.rs-2067 */ impl<FieldIdx: Idx, VariantIdx: Idx> LayoutData<FieldIdx, VariantIdx> {
/* FP:lib.rs-2068 */     /// Returns `true` if this is an aggregate type (including a ScalarPair!)
/* FP:lib.rs-2069 */     pub fn is_aggregate(&self) -> bool {
/* FP:lib.rs-2070 */         match self.backend_repr {
/* FP:lib.rs-2071 */             BackendRepr::Scalar(_) | BackendRepr::SimdVector { .. } => false,
/* FP:lib.rs-2072 */             BackendRepr::ScalarPair(..) | BackendRepr::Memory { .. } => true,
/* FP:lib.rs-2073 */         }
/* FP:lib.rs-2074 */     }
/* FP:lib.rs-2075 */ 
/* FP:lib.rs-2076 */     /// Returns `true` if this is an uninhabited type
/* FP:lib.rs-2077 */     pub fn is_uninhabited(&self) -> bool {
/* FP:lib.rs-2078 */         self.uninhabited
/* FP:lib.rs-2079 */     }
/* FP:lib.rs-2080 */ }
/* FP:lib.rs-2081 */ 
/* FP:lib.rs-2082 */ impl<FieldIdx: Idx, VariantIdx: Idx> fmt::Debug for LayoutData<FieldIdx, VariantIdx>
/* FP:lib.rs-2083 */ where
/* FP:lib.rs-2084 */     FieldsShape<FieldIdx>: fmt::Debug,
/* FP:lib.rs-2085 */     Variants<FieldIdx, VariantIdx>: fmt::Debug,
/* FP:lib.rs-2086 */ {
/* FP:lib.rs-2087 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:lib.rs-2088 */         // This is how `Layout` used to print before it become
/* FP:lib.rs-2089 */         // `Interned<LayoutData>`. We print it like this to avoid having to update
/* FP:lib.rs-2090 */         // expected output in a lot of tests.
/* FP:lib.rs-2091 */         let LayoutData {
/* FP:lib.rs-2092 */             size,
/* FP:lib.rs-2093 */             align,
/* FP:lib.rs-2094 */             backend_repr,
/* FP:lib.rs-2095 */             fields,
/* FP:lib.rs-2096 */             largest_niche,
/* FP:lib.rs-2097 */             uninhabited,
/* FP:lib.rs-2098 */             variants,
/* FP:lib.rs-2099 */             max_repr_align,
/* FP:lib.rs-2100 */             unadjusted_abi_align,
/* FP:lib.rs-2101 */             randomization_seed,
/* FP:lib.rs-2102 */         } = self;
/* FP:lib.rs-2103 */         f.debug_struct("Layout")
/* FP:lib.rs-2104 */             .field("size", size)
/* FP:lib.rs-2105 */             .field("align", align)
/* FP:lib.rs-2106 */             .field("backend_repr", backend_repr)
/* FP:lib.rs-2107 */             .field("fields", fields)
/* FP:lib.rs-2108 */             .field("largest_niche", largest_niche)
/* FP:lib.rs-2109 */             .field("uninhabited", uninhabited)
/* FP:lib.rs-2110 */             .field("variants", variants)
/* FP:lib.rs-2111 */             .field("max_repr_align", max_repr_align)
/* FP:lib.rs-2112 */             .field("unadjusted_abi_align", unadjusted_abi_align)
/* FP:lib.rs-2113 */             .field("randomization_seed", randomization_seed)
/* FP:lib.rs-2114 */             .finish()
/* FP:lib.rs-2115 */     }
/* FP:lib.rs-2116 */ }
/* FP:lib.rs-2117 */ 
/* FP:lib.rs-2118 */ #[derive(Copy, Clone, PartialEq, Eq, Debug)]
/* FP:lib.rs-2119 */ pub enum PointerKind {
/* FP:lib.rs-2120 */     /// Shared reference. `frozen` indicates the absence of any `UnsafeCell`.
/* FP:lib.rs-2121 */     SharedRef { frozen: bool },
/* FP:lib.rs-2122 */     /// Mutable reference. `unpin` indicates the absence of any pinned data.
/* FP:lib.rs-2123 */     MutableRef { unpin: bool },
/* FP:lib.rs-2124 */     /// Box. `unpin` indicates the absence of any pinned data. `global` indicates whether this box
/* FP:lib.rs-2125 */     /// uses the global allocator or a custom one.
/* FP:lib.rs-2126 */     Box { unpin: bool, global: bool },
/* FP:lib.rs-2127 */ }
/* FP:lib.rs-2128 */ 
/* FP:lib.rs-2129 */ /// Encodes extra information we have about a pointer.
/* FP:lib.rs-2130 */ /// Note that this information is advisory only, and backends are free to ignore it:
/* FP:lib.rs-2131 */ /// if the information is wrong, that can cause UB, but if the information is absent,
/* FP:lib.rs-2132 */ /// that must always be okay.
/* FP:lib.rs-2133 */ #[derive(Copy, Clone, Debug)]
/* FP:lib.rs-2134 */ pub struct PointeeInfo {
/* FP:lib.rs-2135 */     /// If this is `None`, then this is a raw pointer, so size and alignment are not guaranteed to
/* FP:lib.rs-2136 */     /// be reliable.
/* FP:lib.rs-2137 */     pub safe: Option<PointerKind>,
/* FP:lib.rs-2138 */     /// If `safe` is `Some`, then the pointer is either null or dereferenceable for this many bytes.
/* FP:lib.rs-2139 */     /// On a function argument, "dereferenceable" here means "dereferenceable for the entire duration
/* FP:lib.rs-2140 */     /// of this function call", i.e. it is UB for the memory that this pointer points to be freed
/* FP:lib.rs-2141 */     /// while this function is still running.
/* FP:lib.rs-2142 */     /// The size can be zero if the pointer is not dereferenceable.
/* FP:lib.rs-2143 */     pub size: Size,
/* FP:lib.rs-2144 */     /// If `safe` is `Some`, then the pointer is aligned as indicated.
/* FP:lib.rs-2145 */     pub align: Align,
/* FP:lib.rs-2146 */ }
/* FP:lib.rs-2147 */ 
/* FP:lib.rs-2148 */ impl<FieldIdx: Idx, VariantIdx: Idx> LayoutData<FieldIdx, VariantIdx> {
/* FP:lib.rs-2149 */     /// Returns `true` if the layout corresponds to an unsized type.
/* FP:lib.rs-2150 */     #[inline]
/* FP:lib.rs-2151 */     pub fn is_unsized(&self) -> bool {
/* FP:lib.rs-2152 */         self.backend_repr.is_unsized()
/* FP:lib.rs-2153 */     }
/* FP:lib.rs-2154 */ 
/* FP:lib.rs-2155 */     #[inline]
/* FP:lib.rs-2156 */     pub fn is_sized(&self) -> bool {
/* FP:lib.rs-2157 */         self.backend_repr.is_sized()
/* FP:lib.rs-2158 */     }
/* FP:lib.rs-2159 */ 
/* FP:lib.rs-2160 */     /// Returns `true` if the type is sized and a 1-ZST (meaning it has size 0 and alignment 1).
/* FP:lib.rs-2161 */     pub fn is_1zst(&self) -> bool {
/* FP:lib.rs-2162 */         self.is_sized() && self.size.bytes() == 0 && self.align.abi.bytes() == 1
/* FP:lib.rs-2163 */     }
/* FP:lib.rs-2164 */ 
/* FP:lib.rs-2165 */     /// Returns `true` if the type is a ZST and not unsized.
/* FP:lib.rs-2166 */     ///
/* FP:lib.rs-2167 */     /// Note that this does *not* imply that the type is irrelevant for layout! It can still have
/* FP:lib.rs-2168 */     /// non-trivial alignment constraints. You probably want to use `is_1zst` instead.
/* FP:lib.rs-2169 */     pub fn is_zst(&self) -> bool {
/* FP:lib.rs-2170 */         match self.backend_repr {
/* FP:lib.rs-2171 */             BackendRepr::Scalar(_)
/* FP:lib.rs-2172 */             | BackendRepr::ScalarPair(..)
/* FP:lib.rs-2173 */             | BackendRepr::SimdVector { .. } => false,
/* FP:lib.rs-2174 */             BackendRepr::Memory { sized } => sized && self.size.bytes() == 0,
/* FP:lib.rs-2175 */         }
/* FP:lib.rs-2176 */     }
/* FP:lib.rs-2177 */ 
/* FP:lib.rs-2178 */     /// Checks if these two `Layout` are equal enough to be considered "the same for all function
/* FP:lib.rs-2179 */     /// call ABIs". Note however that real ABIs depend on more details that are not reflected in the
/* FP:lib.rs-2180 */     /// `Layout`; the `PassMode` need to be compared as well. Also note that we assume
/* FP:lib.rs-2181 */     /// aggregates are passed via `PassMode::Indirect` or `PassMode::Cast`; more strict
/* FP:lib.rs-2182 */     /// checks would otherwise be required.
/* FP:lib.rs-2183 */     pub fn eq_abi(&self, other: &Self) -> bool {
/* FP:lib.rs-2184 */         // The one thing that we are not capturing here is that for unsized types, the metadata must
/* FP:lib.rs-2185 */         // also have the same ABI, and moreover that the same metadata leads to the same size. The
/* FP:lib.rs-2186 */         // 2nd point is quite hard to check though.
/* FP:lib.rs-2187 */         self.size == other.size
/* FP:lib.rs-2188 */             && self.is_sized() == other.is_sized()
/* FP:lib.rs-2189 */             && self.backend_repr.eq_up_to_validity(&other.backend_repr)
/* FP:lib.rs-2190 */             && self.backend_repr.is_bool() == other.backend_repr.is_bool()
/* FP:lib.rs-2191 */             && self.align.abi == other.align.abi
/* FP:lib.rs-2192 */             && self.max_repr_align == other.max_repr_align
/* FP:lib.rs-2193 */             && self.unadjusted_abi_align == other.unadjusted_abi_align
/* FP:lib.rs-2194 */     }
/* FP:lib.rs-2195 */ }
/* FP:lib.rs-2196 */ 
/* FP:lib.rs-2197 */ #[derive(Copy, Clone, Debug)]
/* FP:lib.rs-2198 */ pub enum StructKind {
/* FP:lib.rs-2199 */     /// A tuple, closure, or univariant which cannot be coerced to unsized.
/* FP:lib.rs-2200 */     AlwaysSized,
/* FP:lib.rs-2201 */     /// A univariant, the last field of which may be coerced to unsized.
/* FP:lib.rs-2202 */     MaybeUnsized,
/* FP:lib.rs-2203 */     /// A univariant, but with a prefix of an arbitrary size & alignment (e.g., enum tag).
/* FP:lib.rs-2204 */     Prefixed(Size, Align),
/* FP:lib.rs-2205 */ }
/* FP:lib.rs-2206 */ 
/* FP:lib.rs-2207 */ #[derive(Clone, Debug)]
/* FP:lib.rs-2208 */ pub enum AbiFromStrErr {
/* FP:lib.rs-2209 */     /// not a known ABI
/* FP:lib.rs-2210 */     Unknown,
/* FP:lib.rs-2211 */     /// no "-unwind" variant can be used here
/* FP:lib.rs-2212 */     NoExplicitUnwind,
/* FP:lib.rs-2213 */ }
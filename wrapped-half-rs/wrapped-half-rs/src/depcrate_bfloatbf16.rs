// Generated macro for bf16 (struct)
macro_rules! Depcrate_bfloatbf16 {
() => {
// Module: crate::bfloat
// Provides: {"bf16"}
// Dependencies: {}
# [doc = " A 16-bit floating point type implementing the [`bfloat16`] format."] # [doc = ""] # [doc = " The [`bfloat16`] floating point format is a truncated 16-bit version of the IEEE 754 standard"] # [doc = " `binary32`, a.k.a [`f32`]. [`struct@bf16`] has approximately the same dynamic range as [`f32`] by"] # [doc = " having a lower precision than [`struct@f16`][crate::f16]. While [`struct@f16`][crate::f16] has a precision of"] # [doc = " 11 bits, [`struct@bf16`] has a precision of only 8 bits."] # [doc = ""] # [doc = " [`bfloat16`]: https://en.wikipedia.org/wiki/Bfloat16_floating-point_format"] # [allow (non_camel_case_types)] # [derive (Clone , Copy , Default)] # [repr (transparent)] # [cfg_attr (feature = "serde" , derive (Serialize))] # [cfg_attr (feature = "rkyv" , derive (rkyv :: Archive , rkyv :: Serialize , rkyv :: Deserialize))] # [cfg_attr (feature = "rkyv" , rkyv (resolver = Bf16Resolver))] # [cfg_attr (feature = "bytemuck" , derive (Zeroable , Pod))] # [cfg_attr (kani , derive (kani :: Arbitrary))] # [derive (FromBytes , Immutable , IntoBytes , KnownLayout)] pub struct bf16 (u16) ;
};
}

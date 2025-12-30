// Generated macro for IntrinsicType (struct)
macro_rules! Depcrate_common_intrinsic_helpersIntrinsicType {
() => {
// Module: crate::common::intrinsic_helpers
// Provides: {"IntrinsicType"}
// Dependencies: {}
# [derive (Debug , PartialEq , Clone)] pub struct IntrinsicType { pub constant : bool , # [doc = " whether this object is a const pointer"] pub ptr_constant : bool , pub ptr : bool , pub kind : TypeKind , # [doc = " The bit length of this type (e.g. 32 for u32)."] pub bit_len : Option < u32 > , # [doc = " Length of the SIMD vector (i.e. 4 for uint32x4_t), A value of `None`"] # [doc = " means this is not a simd type. A `None` can be assumed to be 1,"] # [doc = " although in some places a distinction is needed between `u64` and"] # [doc = " `uint64x1_t` this signals that."] pub simd_len : Option < u32 > , # [doc = " The number of rows for SIMD matrices (i.e. 2 for uint8x8x2_t)."] # [doc = " A value of `None` represents a type that does not contain any"] # [doc = " rows encoded in the type (e.g. uint8x8_t)."] # [doc = " A value of `None` can be assumed to be 1 though."] pub vec_len : Option < u32 > , }
};
}

// Generated macro for impl_to_primitive_float (macro)
macro_rules! Depcrate_castimpl_to_primitive_float {
() => {
// Module: crate::cast
// Provides: {"impl_to_primitive_float"}
// Dependencies: {}
macro_rules ! impl_to_primitive_float { ($ T : ident) => { impl ToPrimitive for $ T { impl_to_primitive_float_to_signed_int ! { $ T : fn to_isize -> isize ; fn to_i8 -> i8 ; fn to_i16 -> i16 ; fn to_i32 -> i32 ; fn to_i64 -> i64 ; fn to_i128 -> i128 ; } impl_to_primitive_float_to_unsigned_int ! { $ T : fn to_usize -> usize ; fn to_u8 -> u8 ; fn to_u16 -> u16 ; fn to_u32 -> u32 ; fn to_u64 -> u64 ; fn to_u128 -> u128 ; } impl_to_primitive_float_to_float ! { $ T : fn to_f32 -> f32 ; fn to_f64 -> f64 ; } } } ; }
};
}

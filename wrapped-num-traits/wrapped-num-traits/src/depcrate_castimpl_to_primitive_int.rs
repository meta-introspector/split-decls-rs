// Generated macro for impl_to_primitive_int (macro)
macro_rules! Depcrate_castimpl_to_primitive_int {
() => {
// Module: crate::cast
// Provides: {"impl_to_primitive_int"}
// Dependencies: {}
macro_rules ! impl_to_primitive_int { ($ T : ident) => { impl ToPrimitive for $ T { impl_to_primitive_int_to_int ! { $ T : fn to_isize -> isize ; fn to_i8 -> i8 ; fn to_i16 -> i16 ; fn to_i32 -> i32 ; fn to_i64 -> i64 ; fn to_i128 -> i128 ; } impl_to_primitive_int_to_uint ! { $ T : fn to_usize -> usize ; fn to_u8 -> u8 ; fn to_u16 -> u16 ; fn to_u32 -> u32 ; fn to_u64 -> u64 ; fn to_u128 -> u128 ; } # [inline] fn to_f32 (& self) -> Option < f32 > { Some (* self as f32) } # [inline] fn to_f64 (& self) -> Option < f64 > { Some (* self as f64) } } } ; }
};
}

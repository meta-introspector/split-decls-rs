// Generated macro for impl_to_primitive_nonzero (macro)
macro_rules! Depcrate_castimpl_to_primitive_nonzero {
() => {
// Module: crate::cast
// Provides: {"impl_to_primitive_nonzero"}
// Dependencies: {}
macro_rules ! impl_to_primitive_nonzero { ($ T : ident) => { impl ToPrimitive for $ T { impl_to_primitive_nonzero_to_method ! { $ T : fn to_isize -> isize ; fn to_i8 -> i8 ; fn to_i16 -> i16 ; fn to_i32 -> i32 ; fn to_i64 -> i64 ; fn to_i128 -> i128 ; fn to_usize -> usize ; fn to_u8 -> u8 ; fn to_u16 -> u16 ; fn to_u32 -> u32 ; fn to_u64 -> u64 ; fn to_u128 -> u128 ; fn to_f32 -> f32 ; fn to_f64 -> f64 ; } } } ; }
};
}

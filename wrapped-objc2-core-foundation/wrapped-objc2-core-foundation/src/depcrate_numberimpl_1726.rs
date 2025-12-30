// Generated macro for impl_1726 (impl)
macro_rules! Depcrate_numberimpl_1726 {
() => {
// Module: crate::number
// Provides: {"impl_1726"}
// Dependencies: {}
impl CFNumber { def_get_fn ! { (as_i8 -> i8 ; SInt8Type) , (as_i16 -> i16 ; SInt16Type) , (as_i32 -> i32 ; SInt32Type) , (as_i64 -> i64 ; SInt64Type) , (as_isize -> isize ; NSIntegerType) , (as_f32 -> f32 ; Float32Type) , (as_f64 -> f64 ; Float64Type) , } # [cfg (feature = "CFCGTypes")] # [inline] pub fn as_cgfloat (& self) -> Option < crate :: CGFloat > { # [cfg (not (target_pointer_width = "64"))] { self . as_f32 () } # [cfg (target_pointer_width = "64")] { self . as_f64 () } } }
};
}

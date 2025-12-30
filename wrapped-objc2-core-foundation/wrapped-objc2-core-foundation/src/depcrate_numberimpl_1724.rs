// Generated macro for impl_1724 (impl)
macro_rules! Depcrate_numberimpl_1724 {
() => {
// Module: crate::number
// Provides: {"impl_1724"}
// Dependencies: {}
impl CFNumber { def_new_fn ! { (new_i8 (i8) ; SInt8Type) , (new_i16 (i16) ; SInt16Type) , (new_i32 (i32) ; SInt32Type) , (new_i64 (i64) ; SInt64Type) , (new_isize (isize) ; NSIntegerType) , (new_f32 (f32) ; Float32Type) , (new_f64 (f64) ; Float64Type) , } # [cfg (feature = "CFCGTypes")] # [inline] pub fn new_cgfloat (val : crate :: CGFloat) -> CFRetained < Self > { # [cfg (not (target_pointer_width = "64"))] { Self :: new_f32 (val) } # [cfg (target_pointer_width = "64")] { Self :: new_f64 (val) } } }
};
}

// Generated macro for impl_2289 (impl)
macro_rules! Depcrate_numberimpl_2289 {
() => {
// Module: crate::number
// Provides: {"impl_2289"}
// Dependencies: {}
# [doc = " Creation methods."] impl NSNumber { def_new_fn ! { (new_bool (bool) ; numberWithBool) , (new_i8 (i8) ; numberWithChar) , (new_u8 (u8) ; numberWithUnsignedChar) , (new_i16 (i16) ; numberWithShort) , (new_u16 (u16) ; numberWithUnsignedShort) , (new_i32 (i32) ; numberWithInt) , (new_u32 (u32) ; numberWithUnsignedInt) , (new_i64 (i64) ; numberWithLongLong) , (new_u64 (u64) ; numberWithUnsignedLongLong) , (new_isize (isize) ; numberWithInteger) , (new_usize (usize) ; numberWithUnsignedInteger) , (new_f32 (f32) ; numberWithFloat) , (new_f64 (f64) ; numberWithDouble) , } # [inline] # [cfg (all (feature = "objc2-core-foundation" , feature = "NSGeometry"))] pub fn new_cgfloat (val : objc2_core_foundation :: CGFloat) -> Retained < Self > { # [cfg (target_pointer_width = "64")] { Self :: new_f64 (val) } # [cfg (not (target_pointer_width = "64"))] { Self :: new_f32 (val) } } }
};
}

// Generated macro for other_118 (other)
macro_rules! Depcrate_generatedother_118 {
() => {
// Module: crate::generated
// Provides: {"other_118"}
// Dependencies: {}
extern "C-unwind" { # [doc = " # Safety"] # [doc = ""] # [doc = " - `pix` must be a valid pointer."] # [doc = " - `value` must be a valid pointer."] # [cfg (feature = "CGLTypes")] pub fn CGLDescribePixelFormat (pix : CGLPixelFormatObj , pix_num : GLint , attrib : CGLPixelFormatAttribute , value : NonNull < GLint > ,) -> CGLError ; }
};
}

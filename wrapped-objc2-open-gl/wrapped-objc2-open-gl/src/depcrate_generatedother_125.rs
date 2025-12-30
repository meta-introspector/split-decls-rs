// Generated macro for other_125 (other)
macro_rules! Depcrate_generatedother_125 {
() => {
// Module: crate::generated
// Provides: {"other_125"}
// Dependencies: {}
extern "C-unwind" { # [doc = " # Safety"] # [doc = ""] # [doc = " - `pix` must be a valid pointer."] # [doc = " - `share` must be a valid pointer or null."] # [doc = " - `ctx` must be a valid pointer."] # [cfg (feature = "CGLTypes")] pub fn CGLCreateContext (pix : CGLPixelFormatObj , share : CGLContextObj , ctx : NonNull < CGLContextObj > ,) -> CGLError ; }
};
}

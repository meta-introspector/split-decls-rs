// Generated macro for other_140 (other)
macro_rules! Depcrate_generatedother_140 {
() => {
// Module: crate::generated
// Provides: {"other_140"}
// Dependencies: {}
extern "C-unwind" { # [doc = " # Safety"] # [doc = ""] # [doc = " - `ctx` must be a valid pointer."] # [doc = " - `width` must be a valid pointer."] # [doc = " - `height` must be a valid pointer."] # [doc = " - `rowbytes` must be a valid pointer."] # [doc = " - `baseaddr` must be a valid pointer."] # [cfg (feature = "CGLTypes")] pub fn CGLGetOffScreen (ctx : CGLContextObj , width : NonNull < GLsizei > , height : NonNull < GLsizei > , rowbytes : NonNull < GLint > , baseaddr : NonNull < * mut c_void > ,) -> CGLError ; }
};
}

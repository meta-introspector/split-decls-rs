// Generated macro for other_144 (other)
macro_rules! Depcrate_generatedother_144 {
() => {
// Module: crate::generated
// Provides: {"other_144"}
// Dependencies: {}
extern "C-unwind" { # [doc = " # Safety"] # [doc = ""] # [doc = " - `ctx` must be a valid pointer."] # [doc = " - `pbuffer` must be a valid pointer."] # [doc = " - `face` must be a valid pointer."] # [doc = " - `level` must be a valid pointer."] # [doc = " - `screen` must be a valid pointer."] # [cfg (feature = "CGLTypes")] pub fn CGLGetPBuffer (ctx : CGLContextObj , pbuffer : NonNull < CGLPBufferObj > , face : NonNull < GLenum > , level : NonNull < GLint > , screen : NonNull < GLint > ,) -> CGLError ; }
};
}

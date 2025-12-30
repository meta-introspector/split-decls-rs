// Generated macro for other_134 (other)
macro_rules! Depcrate_generatedother_134 {
() => {
// Module: crate::generated
// Provides: {"other_134"}
// Dependencies: {}
extern "C-unwind" { # [doc = " # Safety"] # [doc = ""] # [doc = " - `obj` must be a valid pointer."] # [doc = " - `width` must be a valid pointer."] # [doc = " - `height` must be a valid pointer."] # [doc = " - `target` must be a valid pointer."] # [doc = " - `internal_format` must be a valid pointer."] # [doc = " - `mipmap` must be a valid pointer."] # [cfg (feature = "CGLTypes")] pub fn CGLDescribePBuffer (obj : CGLPBufferObj , width : NonNull < GLsizei > , height : NonNull < GLsizei > , target : NonNull < GLenum > , internal_format : NonNull < GLenum > , mipmap : NonNull < GLint > ,) -> CGLError ; }
};
}

// Generated macro for other_122 (other)
macro_rules! Depcrate_generatedother_122 {
() => {
// Module: crate::generated
// Provides: {"other_122"}
// Dependencies: {}
extern "C-unwind" { # [doc = " # Safety"] # [doc = ""] # [doc = " - `rend` must be a valid pointer."] # [doc = " - `nrend` must be a valid pointer."] # [cfg (feature = "CGLTypes")] pub fn CGLQueryRendererInfo (display_mask : GLuint , rend : NonNull < CGLRendererInfoObj > , nrend : NonNull < GLint > ,) -> CGLError ; }
};
}

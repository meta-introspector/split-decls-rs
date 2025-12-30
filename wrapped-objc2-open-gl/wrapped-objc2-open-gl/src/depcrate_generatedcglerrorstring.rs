// Generated macro for CGLErrorString (function)
macro_rules! Depcrate_generatedCGLErrorString {
() => {
// Module: crate::generated
// Provides: {"CGLErrorString"}
// Dependencies: {}
# [cfg (feature = "CGLTypes")] # [inline] pub unsafe extern "C-unwind" fn CGLErrorString (error : CGLError) -> NonNull < c_char > { extern "C-unwind" { fn CGLErrorString (error : CGLError) -> Option < NonNull < c_char > > ; } let ret = unsafe { CGLErrorString (error) } ; ret . expect ("function was marked as returning non-null, but actually returned NULL") }
};
}

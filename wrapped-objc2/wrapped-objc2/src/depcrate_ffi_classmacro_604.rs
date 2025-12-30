// Generated macro for macro_604 (macro)
macro_rules! Depcrate_ffi_classmacro_604 {
() => {
// Module: crate::ffi::class
// Provides: {"macro_604"}
// Dependencies: {}
extern_c_unwind ! { # [cfg (any (doc , not (feature = "unstable-objfw")))] pub fn class_getClassMethod (cls : * const AnyClass , name : Sel ,) -> * const Method ; # [cfg (any (doc , not (feature = "unstable-objfw")))] pub fn class_getInstanceMethod (cls : * const AnyClass , name : Sel ,) -> * const Method ; pub fn class_respondsToSelector (cls : * const AnyClass , sel : Sel) -> Bool ; }
};
}

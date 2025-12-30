// Generated macro for impl_35 (impl)
macro_rules! Depcrate_backends_macosimpl_35 {
() => {
// Module: crate::backends::macos
// Provides: {"impl_35"}
// Dependencies: {}
impl CFLocaleWrapper { fn new () -> Option < Self > { let locale = unsafe { CFLocaleCopyCurrent () } ; if locale . is_null () { None } else { Some (CFLocaleWrapper (locale)) } } fn as_ref (& self) -> CFLocaleRef { self . 0 } }
};
}

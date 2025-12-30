// Generated macro for impl_108 (impl)
macro_rules! Depcrate_macro_optionsimpl_108 {
() => {
// Module: crate::macro_options
// Provides: {"impl_108"}
// Dependencies: {}
impl StructLevelSetter { # [doc = " Check if setters are explicitly enabled or disabled at"] # [doc = " the struct level."] pub fn enabled (& self) -> Option < bool > { self . skip . map (| x | ! x) } }
};
}

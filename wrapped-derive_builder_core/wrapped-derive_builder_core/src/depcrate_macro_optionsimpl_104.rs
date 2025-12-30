// Generated macro for impl_104 (impl)
macro_rules! Depcrate_macro_optionsimpl_104 {
() => {
// Module: crate::macro_options
// Provides: {"impl_104"}
// Dependencies: {}
impl BuildFn { fn validation_needs_error (self) -> darling :: Result < Self > { let mut acc = Error :: accumulator () ; if self . validate . is_some () { if let Some (BuildFnError :: Generated (e)) = & self . error { if ! * e . validation_error { acc . push (Error :: custom ("Cannot set `error(validation_error = false)` when using `validate`" ,) . with_span (& e . validation_error . span ()) ,) } } } acc . finish_with (self) } }
};
}

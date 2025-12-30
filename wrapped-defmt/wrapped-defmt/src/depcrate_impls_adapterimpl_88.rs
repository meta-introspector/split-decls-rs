// Generated macro for impl_88 (impl)
macro_rules! Depcrate_impls_adapterimpl_88 {
() => {
// Module: crate::impls::adapter
// Provides: {"impl_88"}
// Dependencies: {}
impl < T : fmt :: Display + ? Sized > Format for Display2Format < '_ , T > { default_format ! () ; fn _format_tag () -> Str { defmt_macros :: internp ! ("{=__internal_Display}") } fn _format_data (& self) { export :: display (& self . 0) ; } }
};
}

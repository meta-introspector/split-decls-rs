// Generated macro for impl_85 (impl)
macro_rules! Depcrate_impls_adapterimpl_85 {
() => {
// Module: crate::impls::adapter
// Provides: {"impl_85"}
// Dependencies: {}
impl < T : fmt :: Debug + ? Sized > Format for Debug2Format < '_ , T > { default_format ! () ; fn _format_tag () -> Str { defmt_macros :: internp ! ("{=__internal_Debug}") } fn _format_data (& self) { export :: debug (& self . 0) ; } }
};
}

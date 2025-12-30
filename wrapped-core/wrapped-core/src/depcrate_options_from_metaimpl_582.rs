// Generated macro for impl_582 (impl)
macro_rules! Depcrate_options_from_metaimpl_582 {
() => {
// Module: crate::options::from_meta
// Provides: {"impl_582"}
// Dependencies: {}
impl < 'a > From < & 'a FromMetaOptions > for FromMetaImpl < 'a > { fn from (v : & 'a FromMetaOptions) -> Self { FromMetaImpl { base : (& v . base) . into () , from_word : v . from_word () , from_none : v . from_none . as_ref () , from_expr : v . from_expr . as_ref () , derive_syn_parse : v . derive_syn_parse . unwrap_or_default () , } } }
};
}

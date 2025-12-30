// Generated macro for impl_277 (impl)
macro_rules! Depcrate_validation_rules_known_fragment_namesimpl_277 {
() => {
// Module: crate::validation::rules::known_fragment_names
// Provides: {"impl_277"}
// Dependencies: {}
impl < 'a > Visitor < 'a > for KnownFragmentNames { fn enter_fragment_spread (& mut self , ctx : & mut VisitorContext < 'a > , fragment_spread : & 'a Positioned < FragmentSpread > ,) { if ! ctx . is_known_fragment (& fragment_spread . node . fragment_name . node) { ctx . report_error (vec ! [fragment_spread . pos] , format ! (r#"Unknown fragment: "{}""# , fragment_spread . node . fragment_name . node) ,) ; } } }
};
}

// Generated macro for impl_785 (impl)
macro_rules! Depcrate_validation_rules_known_fragment_namesimpl_785 {
() => {
// Module: crate::validation::rules::known_fragment_names
// Provides: {"impl_785"}
// Dependencies: {}
impl < 'a , S > Visitor < 'a , S > for KnownFragmentNames where S : ScalarValue , { fn enter_fragment_spread (& mut self , context : & mut ValidatorContext < 'a , S > , spread : & 'a Spanning < FragmentSpread < S > > ,) { let spread_name = & spread . item . name ; if ! context . is_known_fragment (spread_name . item) { context . report_error (& error_message (spread_name . item) , & [spread_name . span . start]) ; } } }
};
}

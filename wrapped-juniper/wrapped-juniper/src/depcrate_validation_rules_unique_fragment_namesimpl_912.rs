// Generated macro for impl_912 (impl)
macro_rules! Depcrate_validation_rules_unique_fragment_namesimpl_912 {
() => {
// Module: crate::validation::rules::unique_fragment_names
// Provides: {"impl_912"}
// Dependencies: {}
impl < 'a , S > Visitor < 'a , S > for UniqueFragmentNames < 'a > where S : ScalarValue , { fn enter_fragment_definition (& mut self , context : & mut ValidatorContext < 'a , S > , f : & 'a Spanning < Fragment < S > > ,) { match self . names . entry (f . item . name . item) { Entry :: Occupied (e) => { context . report_error (& duplicate_message (f . item . name . item) , & [* e . get () , f . item . name . span . start] ,) ; } Entry :: Vacant (e) => { e . insert (f . item . name . span . start) ; } } } }
};
}

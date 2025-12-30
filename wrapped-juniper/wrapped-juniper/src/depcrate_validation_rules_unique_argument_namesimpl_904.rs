// Generated macro for impl_904 (impl)
macro_rules! Depcrate_validation_rules_unique_argument_namesimpl_904 {
() => {
// Module: crate::validation::rules::unique_argument_names
// Provides: {"impl_904"}
// Dependencies: {}
impl < 'a , S > Visitor < 'a , S > for UniqueArgumentNames < 'a > where S : ScalarValue , { fn enter_directive (& mut self , _ : & mut ValidatorContext < 'a , S > , _ : & 'a Spanning < Directive < S > >) { self . known_names = HashMap :: new () ; } fn enter_field (& mut self , _ : & mut ValidatorContext < 'a , S > , _ : & 'a Spanning < Field < S > >) { self . known_names = HashMap :: new () ; } fn enter_argument (& mut self , ctx : & mut ValidatorContext < 'a , S > , (arg_name , _) : & 'a (Spanning < & 'a str > , Spanning < InputValue < S > >) ,) { match self . known_names . entry (arg_name . item) { Entry :: Occupied (e) => { ctx . report_error (& error_message (arg_name . item) , & [* e . get () , arg_name . span . start] ,) ; } Entry :: Vacant (e) => { e . insert (arg_name . span . start) ; } } } }
};
}

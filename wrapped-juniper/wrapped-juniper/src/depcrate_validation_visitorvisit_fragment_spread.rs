// Generated macro for visit_fragment_spread (function)
macro_rules! Depcrate_validation_visitorvisit_fragment_spread {
() => {
// Module: crate::validation::visitor
// Provides: {"visit_fragment_spread"}
// Dependencies: {}
fn visit_fragment_spread < 'a , S , V > (v : & mut V , ctx : & mut ValidatorContext < 'a , S > , spread : & 'a Spanning < FragmentSpread < S > > ,) where S : ScalarValue , V : Visitor < 'a , S > , { v . enter_fragment_spread (ctx , spread) ; visit_directives (v , ctx , & spread . item . directives) ; v . exit_fragment_spread (ctx , spread) ; }
};
}

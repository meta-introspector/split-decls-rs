// Generated macro for visit_selection (function)
macro_rules! Depcrate_validation_visitorvisit_selection {
() => {
// Module: crate::validation::visitor
// Provides: {"visit_selection"}
// Dependencies: {}
fn visit_selection < 'a , S , V > (v : & mut V , ctx : & mut ValidatorContext < 'a , S > , selection : & 'a Selection < S > ,) where S : ScalarValue , V : Visitor < 'a , S > , { match * selection { Selection :: Field (ref field) => visit_field (v , ctx , field) , Selection :: FragmentSpread (ref spread) => visit_fragment_spread (v , ctx , spread) , Selection :: InlineFragment (ref fragment) => visit_inline_fragment (v , ctx , fragment) , } }
};
}

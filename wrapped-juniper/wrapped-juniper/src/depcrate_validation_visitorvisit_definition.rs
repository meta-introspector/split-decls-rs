// Generated macro for visit_definition (function)
macro_rules! Depcrate_validation_visitorvisit_definition {
() => {
// Module: crate::validation::visitor
// Provides: {"visit_definition"}
// Dependencies: {}
fn visit_definition < 'a , S , V > (v : & mut V , ctx : & mut ValidatorContext < 'a , S > , def : & 'a Definition < S >) where S : ScalarValue , V : Visitor < 'a , S > , { match * def { Definition :: Operation (ref op) => { visit_variables_definition (v , ctx , & op . item . variables_definition) ; visit_directives (v , ctx , & op . item . directives) ; visit_selection_set (v , ctx , & op . item . selection_set) ; } Definition :: Fragment (ref f) => { visit_directives (v , ctx , & f . item . directives) ; visit_selection_set (v , ctx , & f . item . selection_set) ; } } }
};
}

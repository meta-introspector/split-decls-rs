// Generated macro for visit_field (function)
macro_rules! Depcrate_validation_visitorvisit_field {
() => {
// Module: crate::validation::visitor
// Provides: {"visit_field"}
// Dependencies: {}
fn visit_field < 'a , S , V > (v : & mut V , ctx : & mut ValidatorContext < 'a , S > , field : & 'a Spanning < Field < S > > ,) where S : ScalarValue , V : Visitor < 'a , S > , { let meta_field = ctx . parent_type () . and_then (| t | t . field_by_name (field . item . name . item)) ; let field_type = meta_field . map (| f | & f . field_type) ; let field_args = meta_field . and_then (| f | f . arguments . as_ref ()) ; ctx . with_pushed_type (field_type , | ctx | { v . enter_field (ctx , field) ; visit_arguments (v , ctx , field_args , & field . item . arguments) ; visit_directives (v , ctx , & field . item . directives) ; if let Some (ref selection_set) = field . item . selection_set { visit_selection_set (v , ctx , selection_set) ; } v . exit_field (ctx , field) ; }) ; }
};
}

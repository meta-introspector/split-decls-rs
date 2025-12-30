// Generated macro for visit_field (function)
macro_rules! Depcrate_validation_visitorvisit_field {
() => {
// Module: crate::validation::visitor
// Provides: {"visit_field"}
// Dependencies: {}
fn visit_field < 'a , V : Visitor < 'a > > (v : & mut V , ctx : & mut VisitorContext < 'a > , field : & 'a Positioned < Field > ,) { v . enter_field (ctx , field) ; for (name , value) in & field . node . arguments { v . enter_argument (ctx , name , value) ; let expected_ty = ctx . parent_type () . and_then (| ty | ty . field_by_name (& field . node . name . node)) . and_then (| schema_field | schema_field . args . get (& * name . node)) . map (| input_ty | MetaTypeName :: create (& input_ty . ty)) ; ctx . with_input_type (expected_ty , | ctx | { visit_input_value (v , ctx , field . pos , expected_ty , & value . node) }) ; v . exit_argument (ctx , name , value) ; } visit_directives (v , ctx , & field . node . directives) ; visit_selection_set (v , ctx , & field . node . selection_set) ; v . exit_field (ctx , field) ; }
};
}

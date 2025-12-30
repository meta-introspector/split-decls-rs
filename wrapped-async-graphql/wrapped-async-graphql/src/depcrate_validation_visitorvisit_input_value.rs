// Generated macro for visit_input_value (function)
macro_rules! Depcrate_validation_visitorvisit_input_value {
() => {
// Module: crate::validation::visitor
// Provides: {"visit_input_value"}
// Dependencies: {}
fn visit_input_value < 'a , V : Visitor < 'a > > (v : & mut V , ctx : & mut VisitorContext < 'a > , pos : Pos , expected_ty : Option < MetaTypeName < 'a > > , value : & 'a Value ,) { v . enter_input_value (ctx , pos , & expected_ty , value) ; match value { Value :: List (values) => { if let Some (expected_ty) = expected_ty { let elem_ty = expected_ty . unwrap_non_null () ; if let MetaTypeName :: List (expected_ty) = elem_ty { values . iter () . for_each (| value | { visit_input_value (v , ctx , pos , Some (MetaTypeName :: create (expected_ty)) , value ,) }) ; } } } Value :: Object (values) => { if let Some (expected_ty) = expected_ty { let expected_ty = expected_ty . unwrap_non_null () ; if let MetaTypeName :: Named (expected_ty) = expected_ty { if let Some (MetaType :: InputObject { input_fields , .. }) = ctx . registry . types . get (MetaTypeName :: concrete_typename (expected_ty)) { for (item_key , item_value) in values { if let Some (input_value) = input_fields . get (item_key . as_str ()) { visit_input_value (v , ctx , pos , Some (MetaTypeName :: create (& input_value . ty)) , item_value ,) ; } } } } } } _ => { } } v . exit_input_value (ctx , pos , & expected_ty , value) ; }
};
}

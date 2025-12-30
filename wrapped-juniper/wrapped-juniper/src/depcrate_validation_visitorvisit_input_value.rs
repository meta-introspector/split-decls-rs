// Generated macro for visit_input_value (function)
macro_rules! Depcrate_validation_visitorvisit_input_value {
() => {
// Module: crate::validation::visitor
// Provides: {"visit_input_value"}
// Dependencies: {}
fn visit_input_value < 'a , S , V > (v : & mut V , ctx : & mut ValidatorContext < 'a , S > , input_value : & 'a Spanning < InputValue < S > > ,) where S : ScalarValue , V : Visitor < 'a , S > , { enter_input_value (v , ctx , input_value) ; match & input_value . item { InputValue :: Object (fields) => { for (key , value) in fields { let inner_type = ctx . current_input_type_literal () . and_then (| t | t . name () . and_then (| n | ctx . schema . concrete_type_by_name (n))) . and_then (| ct | ct . input_field_by_name (& key . item)) . map (| f | & f . arg_type) ; ctx . with_pushed_input_type (inner_type , | ctx | { v . enter_object_field (ctx , (key . as_ref () , value . as_ref ())) ; visit_input_value (v , ctx , value) ; v . exit_object_field (ctx , (key . as_ref () , value . as_ref ())) ; }) } } InputValue :: List (ls) => { let inner_type = ctx . current_input_type_literal () . and_then (| t | t . borrow_list_inner ()) ; ctx . with_pushed_input_type (inner_type , | ctx | { for value in ls { visit_input_value (v , ctx , value) ; } }) } _ => () , } exit_input_value (v , ctx , input_value) ; }
};
}

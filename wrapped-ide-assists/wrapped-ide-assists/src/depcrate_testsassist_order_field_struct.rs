// Generated macro for assist_order_field_struct (function)
macro_rules! Depcrate_testsassist_order_field_struct {
() => {
// Module: crate::tests
// Provides: {"assist_order_field_struct"}
// Dependencies: {}
# [test] fn assist_order_field_struct () { let before = "struct Foo { $0bar: u32 }" ; let (before_cursor_pos , before) = extract_offset (before) ; let (db , file_id) = with_single_file (& before) ; let frange = FileRange { file_id : file_id . file_id (& db) , range : TextRange :: empty (before_cursor_pos) } ; let assists = assists (& db , & TEST_CONFIG , AssistResolveStrategy :: None , frange) ; let mut assists = assists . iter () ; assert_eq ! (assists . next () . expect ("expected assist") . label , "Change visibility to pub(crate)") ; assert_eq ! (assists . next () . expect ("expected assist") . label , "Generate a getter method") ; assert_eq ! (assists . next () . expect ("expected assist") . label , "Generate a mut getter method") ; assert_eq ! (assists . next () . expect ("expected assist") . label , "Generate a setter method") ; assert_eq ! (assists . next () . expect ("expected assist") . label , "Add `#[derive]`") ; assert_eq ! (assists . next () . expect ("expected assist") . label , "Generate `new`") ; assert_eq ! (assists . next () . map (| it | it . label . to_string ()) , None) ; }
};
}

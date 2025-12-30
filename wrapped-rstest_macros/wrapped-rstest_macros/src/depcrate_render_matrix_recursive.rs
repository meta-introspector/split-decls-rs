// Generated macro for _matrix_recursive (function)
macro_rules! Depcrate_render_matrix_recursive {
() => {
// Module: crate::render
// Provides: {"_matrix_recursive"}
// Dependencies: {}
fn _matrix_recursive < 'a > (test : & ItemFn , list_values : & 'a [& 'a ValueList] , resolver : & dyn Resolver , attrs : & 'a [syn :: Attribute] , info : & RsTestInfo , case_info : & Option < CaseInfo > ,) -> TokenStream { if list_values . is_empty () { return Default :: default () ; } let vlist = list_values [0] ; let list_values = & list_values [1 ..] ; if list_values . is_empty () { let mut attrs = attrs . to_vec () ; attrs . push (parse_quote ! (# [allow (non_snake_case)])) ; vlist . render (test , resolver , & attrs , info , case_info) } else { let span = test . sig . ident . span () ; let modules = vlist . argument_data (resolver , info) . map (move | (name , resolver) | { _matrix_recursive (test , list_values , & resolver , attrs , info , case_info) . wrap_by_mod (& Ident :: new (& name , span)) }) ; quote ! { # (# [allow (non_snake_case)] # modules) * } } }
};
}

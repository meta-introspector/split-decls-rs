// Generated macro for iai (function)
macro_rules! Depcrateiai {
() => {
// Module: crate
// Provides: {"iai"}
// Dependencies: {}
# [proc_macro_attribute] pub fn iai (_attr : TokenStream , item : TokenStream) -> TokenStream { let item = proc_macro2 :: TokenStream :: from (item) ; let span = proc_macro2 :: Span :: call_site () ; let function_name = find_name (item . clone ()) ; let wrapper_function_name = Ident :: new (& format ! ("wrap_{}" , function_name . to_string ()) , span) ; let const_name = Ident :: new (& format ! ("IAI_FUNC_{}" , function_name . to_string ()) , span) ; let name_literal = function_name . to_string () ; let output = quote_spanned ! (span => # item fn # wrapper_function_name () { let _ = iai :: black_box (# function_name ()) ; } # [test_case] const # const_name : (&'static str , fn ()) = (# name_literal , # wrapper_function_name) ;) ; output . into () }
};
}

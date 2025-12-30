// Generated macro for create_helper_macro (function)
macro_rules! Depcrate_arbitrary_in_macro_rulescreate_helper_macro {
() => {
// Module: crate::arbitrary_in_macro_rules
// Provides: {"create_helper_macro"}
// Dependencies: {}
fn create_helper_macro (macro_name : & str , traits : & [String]) -> Item { let macro_ident = syn :: Ident :: new (macro_name , proc_macro2 :: Span :: call_site ()) ; let trait_list = traits . iter () . map (| t | syn :: Ident :: new (t , proc_macro2 :: Span :: call_site ())) . collect :: < Vec < _ > > () ; parse_quote ! { macro_rules ! # macro_ident { ($ item : item) => { # [derive (# (# trait_list) ,*)] $ item } ; } } }
};
}

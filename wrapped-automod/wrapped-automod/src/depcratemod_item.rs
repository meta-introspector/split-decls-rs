// Generated macro for mod_item (function)
macro_rules! Depcratemod_item {
() => {
// Module: crate
// Provides: {"mod_item"}
// Dependencies: {}
fn mod_item (vis : & Visibility , name : String) -> TokenStream2 { let mut module_name = name . replace ('-' , "_") ; if module_name . starts_with (| ch : char | ch . is_ascii_digit ()) { module_name . insert (0 , '_') ; } let path = Option :: into_iter (if name == module_name { None } else { Some (format ! ("{}.rs" , name)) }) ; let ident = Ident :: new (& module_name , Span :: call_site ()) ; quote ! { # (# [path = # path]) * # vis mod # ident ; } }
};
}

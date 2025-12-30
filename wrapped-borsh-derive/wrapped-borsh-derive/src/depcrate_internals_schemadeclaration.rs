// Generated macro for declaration (function)
macro_rules! Depcrate_internals_schemadeclaration {
() => {
// Module: crate::internals::schema
// Provides: {"declaration"}
// Dependencies: {}
fn declaration (ident_str : & str , cratename : Path , params_for_bounds : Vec < Type >) -> TokenStream2 { let mut declaration_params = vec ! [] ; for type_param in params_for_bounds { declaration_params . push (quote ! { <# type_param as # cratename :: BorshSchema >:: declaration () }) ; } if declaration_params . is_empty () { quote ! { # ident_str . to_string () } } else { quote ! { let params = # cratename :: __private :: maybestd :: vec ! [# (# declaration_params) ,*] ; format ! (r#"{}<{}>"# , # ident_str , params . join (", ")) } } }
};
}

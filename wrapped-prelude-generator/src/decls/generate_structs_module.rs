macro_rules! generate_structs_module {
    () => {
        pub fn generate_structs_module (structs : & [syn :: ItemStruct]) -> String { let generated_decl_strings : Vec < String > = structs . iter () . map (| s | { let tokens = quote ! { # s } ; tokens . to_string () }) . collect () ; if generated_decl_strings . is_empty () { return "// No struct declarations found in this module.\n" . to_string () ; } let header = "// This module contains extracted struct declarations.\n// It is automatically generated.\n\n" ; let joined_decls = generated_decl_strings . join ("\n\n") ; format ! ("{}{}" , header , joined_decls) }
    };
}

generate_structs_module!()
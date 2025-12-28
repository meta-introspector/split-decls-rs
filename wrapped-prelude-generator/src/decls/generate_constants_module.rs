macro_rules! generate_constants_module {
    () => {
        pub fn generate_constants_module (constants : & [syn :: ItemConst]) -> String { let generated_decl_strings : Vec < String > = constants . iter () . map (| c | { let tokens = quote ! { # c } ; tokens . to_string () }) . collect () ; if generated_decl_strings . is_empty () { return "// No constant declarations found in this module.\n" . to_string () ; } let header = "// This module contains extracted constant declarations.\n// It is automatically generated.\n\n" ; let joined_decls = generated_decl_strings . join ("\n\n") ; format ! ("{}{}" , header , joined_decls) }
    };
}

generate_constants_module!();
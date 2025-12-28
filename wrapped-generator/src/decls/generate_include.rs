macro_rules! generate_include {
    () => {
        # [doc = " Generate Rust `include_str!` for grammar files, then Cargo will watch changes in grammars."] fn generate_include (name : & Ident , paths : Vec < PathBuf >) -> TokenStream { let const_name = format_ident ! ("_PEST_GRAMMAR_{}" , name) ; let current_dir = std :: env :: current_dir () . expect ("Unable to get current directory") ; let include_tokens = paths . iter () . map (| path | { let path = path . to_str () . expect ("non-Unicode path") ; let relative_path = current_dir . join (path) . to_str () . expect ("path contains invalid unicode") . to_string () ; quote ! { include_str ! (# relative_path) } }) ; let len = include_tokens . len () ; quote ! { # [allow (non_upper_case_globals)] const # const_name : [&'static str ; # len] = [# (# include_tokens) ,*] ; } }
    };
}

generate_include!()
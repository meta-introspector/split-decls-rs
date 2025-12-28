macro_rules! deps {
    () => {
        KeyValue!();
        InlineTable!();
        TomlSection!();
    };
}

macro_rules! parse_dependencies_map {
    () => {
        deps!();
        pub fn parse_dependencies_map (input : proc_macro2 :: TokenStream ,) -> Result < HashMap < String , Dependency > > { let toml_section : TomlSection = syn :: parse2 (input) ? ; let mut deps = HashMap :: new () ; for item in toml_section . items { match item { KeyValue :: Simple (key , val) => { deps . insert (key . to_string () , Dependency :: Version (val . value ())) ; } KeyValue :: InlineTable (key , table_tokens) => { deps . insert (key . to_string () , Dependency :: Table (parse_dependency_table (table_tokens) ?) ,) ; } _ => { return Err (syn :: Error :: new_spanned (item . get_ident_for_err () , "expected simple version string or inline table for dependency" ,)) } } } Ok (deps) }
    };
}

parse_dependencies_map!();
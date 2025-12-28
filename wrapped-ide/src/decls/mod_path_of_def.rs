macro_rules! mod_path_of_def {
    () => {
        fn mod_path_of_def (db : & RootDatabase , def : Definition) -> Option < String > { def . canonical_module_path (db) . map (| it | { let mut path = String :: new () ; it . flat_map (| it | it . name (db)) . for_each (| name | format_to ! (path , "{}/" , name . as_str ())) ; path }) }
    };
}

mod_path_of_def!();
macro_rules! normalize_expansion {
    () => {
        fn normalize_expansion (input : & [u8]) -> String { let code = String :: from_utf8_lossy (input) ; let mut syntax_tree = match syn :: parse_file (& code) { Ok (syntax_tree) => syntax_tree , Err (_) => return code . into_owned () , } ; syntax_tree . attrs . retain (| attr | { if let Meta :: List (meta) = & attr . meta { if meta . path . is_ident ("feature") { if let Ok (list) = meta . parse_args_with (Punctuated :: < Meta , Token ! [,] > :: parse_terminated) { if list . len () == 1 { if let Meta :: Path (inner) = & list . first () . unwrap () { if inner . is_ident ("prelude_import") { return false ; } } } } } } true }) ; syntax_tree . items . retain (| item | { if let Item :: Use (item) = item { if let Some (attr) = item . attrs . first () { if attr . path () . is_ident ("prelude_import") && attr . meta . require_path_only () . is_ok () { return false ; } } } if let Item :: ExternCrate (item) = item { if item . ident == "std" { return false ; } } true }) ; prettyplease :: unparse (& syntax_tree) }
    };
}

normalize_expansion!()
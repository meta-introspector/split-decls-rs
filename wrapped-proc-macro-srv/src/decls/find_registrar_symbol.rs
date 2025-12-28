macro_rules! find_registrar_symbol {
    () => {
        fn find_registrar_symbol (obj : & object :: File < '_ >) -> object :: Result < Option < String > > { Ok (obj . exports () ? . into_iter () . map (| export | export . name ()) . filter_map (| sym | String :: from_utf8 (sym . into ()) . ok ()) . find (| sym | is_derive_registrar_symbol (sym)) . map (| sym | { if cfg ! (target_os = "macos") && sym . starts_with ('_') { sym [1 ..] . to_owned () } else { sym } })) }
    };
}

find_registrar_symbol!()
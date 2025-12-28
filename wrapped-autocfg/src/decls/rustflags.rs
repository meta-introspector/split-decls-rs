macro_rules! rustflags {
    () => {
        fn rustflags (target : & Option < OsString > , dir : & Path) -> Vec < String > { if let Ok (a) = env :: var ("CARGO_ENCODED_RUSTFLAGS") { return if a . is_empty () { Vec :: new () } else { a . split ('\x1f') . map (str :: to_string) . collect () } ; } if * target != env :: var_os ("HOST") || dir_contains_target (target , dir , env :: var_os ("CARGO_TARGET_DIR")) { if let Ok (rustflags) = env :: var ("RUSTFLAGS") { return rustflags . split (' ') . map (str :: trim) . filter (| s | ! s . is_empty ()) . map (str :: to_string) . collect () ; } } Vec :: new () }
    };
}

rustflags!();
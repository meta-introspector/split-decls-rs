macro_rules! test_edition_parsing {
    () => {
        # [test] fn test_edition_parsing () { let options = Options :: default () ; assert ! (options . edition == DEFAULT_EDITION) ; let mut early_dcx = EarlyDiagCtxt :: new (ErrorOutputType :: default ()) ; let matches = optgroups () . parse (& ["--edition=2018" . to_string ()]) . unwrap () ; let sessopts = build_session_options (& mut early_dcx , & matches) ; assert ! (sessopts . edition == Edition :: Edition2018) }
    };
}

test_edition_parsing!()
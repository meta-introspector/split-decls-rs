macro_rules! test {
    () => {
        # [cfg (test)] mod test { # [test] fn version () { if let Ok (version) = super :: version () { assert ! (version . major >= 4) ; } else { println ! ("Gnuplot not installed.") ; } } # [test] fn test_parse_version_on_valid_string () { let string = "gnuplot 5.0 patchlevel 7" ; let version = super :: parse_version (string) . unwrap () ; assert_eq ! (5 , version . major) ; assert_eq ! (0 , version . minor) ; assert_eq ! ("7" , & version . patch) ; } # [test] fn test_parse_gentoo_version () { let string = "gnuplot 5.2 patchlevel 5a (Gentoo revision r0)" ; let version = super :: parse_version (string) . unwrap () ; assert_eq ! (5 , version . major) ; assert_eq ! (2 , version . minor) ; assert_eq ! ("5a" , & version . patch) ; } # [test] fn test_parse_version_returns_error_on_invalid_strings () { let strings = ["" , "foobar" , "gnuplot 50 patchlevel 7" , "gnuplot 5.0 patchlevel" , "gnuplot foo.bar patchlevel 7" ,] ; for string in & strings { assert ! (super :: parse_version (string) . is_err ()) ; } } }
    };
}

test!();
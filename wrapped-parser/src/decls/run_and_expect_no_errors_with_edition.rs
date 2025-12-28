macro_rules! deps {
    () => {
        TopEntryPoint!();
    };
}

macro_rules! run_and_expect_no_errors_with_edition {
    () => {
        deps!();
        # [track_caller] fn run_and_expect_no_errors_with_edition (path : & str , edition : Edition) { let path = PathBuf :: from (path) ; let text = std :: fs :: read_to_string (& path) . unwrap () ; let (actual , errors) = parse (TopEntryPoint :: SourceFile , & text , edition) ; assert ! (! errors , "errors in an OK file {}:\n{actual}" , path . display ()) ; let mut p = PathBuf :: from ("..") ; p . push (path) ; p . set_extension ("rast") ; expect_file ! [p] . assert_eq (& actual) }
    };
}

run_and_expect_no_errors_with_edition!();
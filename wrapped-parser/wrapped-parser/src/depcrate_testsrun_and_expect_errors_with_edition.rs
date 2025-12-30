// Generated macro for run_and_expect_errors_with_edition (function)
macro_rules! Depcrate_testsrun_and_expect_errors_with_edition {
() => {
// Module: crate::tests
// Provides: {"run_and_expect_errors_with_edition"}
// Dependencies: {}
# [track_caller] fn run_and_expect_errors_with_edition (path : & str , edition : Edition) { let path = PathBuf :: from (path) ; let text = std :: fs :: read_to_string (& path) . unwrap () ; let (actual , errors) = parse (TopEntryPoint :: SourceFile , & text , edition) ; assert ! (errors , "no errors in an ERR file {}:\n{actual}" , path . display ()) ; let mut p = PathBuf :: from ("..") ; p . push (path) ; p . set_extension ("rast") ; expect_file ! [p] . assert_eq (& actual) }
};
}

// Generated macro for check_diff (function)
macro_rules! Depcratecheck_diff {
() => {
// Module: crate
// Provides: {"check_diff"}
// Dependencies: {}
# [doc = " Calculates the number of errors when running the compiled binary and the feature binary on the"] # [doc = " repo specified with the specific configs."] pub fn check_diff (config : Option < Vec < String > > , runners : CheckDiffRunners < impl CodeFormatter , impl CodeFormatter > , repo : & Path ,) -> i32 { let mut errors = 0 ; let iter = search_for_rs_files (repo) ; for file in iter { match runners . create_diff (file . as_path () , & config) { Ok (diff) => { if ! diff . is_empty () { eprint ! ("{diff}") ; errors += 1 ; } } Err (e) => { eprintln ! ("Error creating diff for {:?}: {:?}" , file . as_path () . display () , e) ; errors += 1 ; } } } return errors ; }
};
}

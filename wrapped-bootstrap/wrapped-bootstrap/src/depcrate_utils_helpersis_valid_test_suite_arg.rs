// Generated macro for is_valid_test_suite_arg (function)
macro_rules! Depcrate_utils_helpersis_valid_test_suite_arg {
() => {
// Module: crate::utils::helpers
// Provides: {"is_valid_test_suite_arg"}
// Dependencies: {}
pub fn is_valid_test_suite_arg < 'a , P : AsRef < Path > > (path : & 'a Path , suite_path : P , builder : & Builder < '_ > ,) -> Option < & 'a str > { let suite_path = suite_path . as_ref () ; let path = match path . strip_prefix (".") { Ok (p) => p , Err (_) => path , } ; if ! path . starts_with (suite_path) { return None ; } let abs_path = builder . src . join (path) ; let exists = abs_path . is_dir () || abs_path . is_file () ; if ! exists { panic ! ("Invalid test suite filter \"{}\": file or directory does not exist" , abs_path . display ()) ; } match path . strip_prefix (suite_path) . ok () . and_then (| p | p . to_str ()) { Some (s) if ! s . is_empty () => Some (s) , _ => None , } }
};
}

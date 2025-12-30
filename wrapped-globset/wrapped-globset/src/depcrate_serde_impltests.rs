// Generated macro for tests (module)
macro_rules! Depcrate_serde_impltests {
() => {
// Module: crate::serde_impl
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: collections :: HashMap ; use crate :: { Glob , GlobSet } ; # [test] fn glob_deserialize_borrowed () { let string = r#"{"markdown": "*.md"}"# ; let map : HashMap < String , Glob > = serde_json :: from_str (& string) . unwrap () ; assert_eq ! (map ["markdown"] , Glob :: new ("*.md") . unwrap ()) ; } # [test] fn glob_deserialize_owned () { let string = r#"{"markdown": "*.md"}"# ; let v : serde_json :: Value = serde_json :: from_str (& string) . unwrap () ; let map : HashMap < String , Glob > = serde_json :: from_value (v) . unwrap () ; assert_eq ! (map ["markdown"] , Glob :: new ("*.md") . unwrap ()) ; } # [test] fn glob_deserialize_error () { let string = r#"{"error": "["}"# ; let map = serde_json :: from_str :: < HashMap < String , Glob > > (& string) ; assert ! (map . is_err ()) ; } # [test] fn glob_json_works () { let test_glob = Glob :: new ("src/**/*.rs") . unwrap () ; let ser = serde_json :: to_string (& test_glob) . unwrap () ; assert_eq ! (ser , "\"src/**/*.rs\"") ; let de : Glob = serde_json :: from_str (& ser) . unwrap () ; assert_eq ! (test_glob , de) ; } # [test] fn glob_set_deserialize () { let j = r#" ["src/**/*.rs", "README.md"] "# ; let set : GlobSet = serde_json :: from_str (j) . unwrap () ; assert ! (set . is_match ("src/lib.rs")) ; assert ! (! set . is_match ("Cargo.lock")) ; } }
};
}

// Generated macro for tests (module)
macro_rules! Depcrate_registrytests {
() => {
// Module: crate::registry
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: make_dep_path ; # [test] fn prefix_only () { assert_eq ! (make_dep_path ("a" , true) , "1") ; assert_eq ! (make_dep_path ("ab" , true) , "2") ; assert_eq ! (make_dep_path ("abc" , true) , "3/a") ; assert_eq ! (make_dep_path ("Abc" , true) , "3/A") ; assert_eq ! (make_dep_path ("AbCd" , true) , "Ab/Cd") ; assert_eq ! (make_dep_path ("aBcDe" , true) , "aB/cD") ; } # [test] fn full () { assert_eq ! (make_dep_path ("a" , false) , "1/a") ; assert_eq ! (make_dep_path ("ab" , false) , "2/ab") ; assert_eq ! (make_dep_path ("abc" , false) , "3/a/abc") ; assert_eq ! (make_dep_path ("Abc" , false) , "3/A/Abc") ; assert_eq ! (make_dep_path ("AbCd" , false) , "Ab/Cd/AbCd") ; assert_eq ! (make_dep_path ("aBcDe" , false) , "aB/cD/aBcDe") ; } }
};
}

// Generated macro for sig_split (function)
macro_rules! Depcratesig_split {
() => {
// Module: crate
// Provides: {"sig_split"}
// Dependencies: {}
# [test] fn sig_split () { let s = SignatureMulti :: new ("ua{sv}(ss)") . unwrap () ; let (a2 , s2) = s . single () . unwrap () ; assert_eq ! (&** a2 , "u") ; assert_eq ! (&** s2 , "a{sv}(ss)") ; let (a3 , s3) = s2 . single () . unwrap () ; assert_eq ! (&** a3 , "a{sv}") ; assert_eq ! (&** s3 , "(ss)") ; let (a4 , s4) = s3 . single () . unwrap () ; assert_eq ! (&** a4 , "(ss)") ; assert_eq ! (&** s4 , "") ; assert ! (s4 . single () . is_none ()) ; }
};
}

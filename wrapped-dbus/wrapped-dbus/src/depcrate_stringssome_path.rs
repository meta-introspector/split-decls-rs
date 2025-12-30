// Generated macro for some_path (function)
macro_rules! Depcrate_stringssome_path {
() => {
// Module: crate::strings
// Provides: {"some_path"}
// Dependencies: {}
# [test] fn some_path () { let p1 : Path = "/valid" . into () ; let p2 = Path :: new ("##invalid##") ; assert_eq ! (p1 , Path (Cow :: Borrowed ("/valid\0"))) ; # [cfg (not (feature = "no-string-validation"))] assert_eq ! (p2 , Err ("Object path was not valid: '##invalid##'" . into ())) ; # [cfg (feature = "no-string-validation")] assert_eq ! (p2 , Ok (Path (Cow :: Borrowed ("##invalid##\0")))) ; }
};
}

// Generated macro for invalid_create_params (function)
macro_rules! Depcrateinvalid_create_params {
() => {
// Module: crate
// Provides: {"invalid_create_params"}
// Dependencies: {}
# [test] fn invalid_create_params () { unsafe { let schema = r#"{ "invalid"# ; let mut handle = 0 ; let code = CreateJsonValidator (schema . as_ptr () , schema . len () , & mut handle) ; assert_eq ! (E_INVALIDARG , code) ; assert_eq ! ("EOF while parsing a string at line 1 column 10" , Error :: from (code) . message ()) ; let schema = r#"{"maxLength": 5}"# ; let mut handle = 0 ; assert_eq ! (E_POINTER , CreateJsonValidator (std :: ptr :: null () , schema . len () , & mut handle)) ; assert_eq ! (E_POINTER , CreateJsonValidator (schema . as_ptr () , schema . len () , std :: ptr :: null_mut ())) ; } }
};
}

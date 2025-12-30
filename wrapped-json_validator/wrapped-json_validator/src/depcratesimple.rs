// Generated macro for simple (function)
macro_rules! Depcratesimple {
() => {
// Module: crate
// Provides: {"simple"}
// Dependencies: {}
# [test] fn simple () { unsafe { let schema = r#"{"maxLength": 5}"# ; let mut handle = 0 ; assert_eq ! (S_OK , CreateJsonValidator (schema . as_ptr () , schema . len () , & mut handle)) ; let value = r#""Hello""# ; assert_eq ! (S_OK , ValidateJson (handle , value . as_ptr () , value . len () , std :: ptr :: null_mut () , std :: ptr :: null_mut ())) ; let value = r#""Hello World""# ; let code = ValidateJson (handle , value . as_ptr () , value . len () , std :: ptr :: null_mut () , std :: ptr :: null_mut () ,) ; assert_eq ! (E_INVALIDARG , code) ; assert_eq ! (r#""Hello World" is longer than 5 characters"# , Error :: from (code) . message ()) ; let value = r#""World""# ; assert_eq ! (S_OK , ValidateJson (handle , value . as_ptr () , value . len () , std :: ptr :: null_mut () , std :: ptr :: null_mut ())) ; CloseJsonValidator (handle) ; CloseJsonValidator (0) ; } }
};
}

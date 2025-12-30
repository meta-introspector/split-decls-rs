// Generated macro for invalid_validate_params (function)
macro_rules! Depcrateinvalid_validate_params {
() => {
// Module: crate
// Provides: {"invalid_validate_params"}
// Dependencies: {}
# [test] fn invalid_validate_params () { unsafe { let schema = r#"{"maxLength": 5}"# ; let mut handle = 0 ; assert_eq ! (S_OK , CreateJsonValidator (schema . as_ptr () , schema . len () , & mut handle)) ; let value = r#""Hello""# ; assert_eq ! (E_HANDLE , ValidateJson (0 , value . as_ptr () , value . len () , std :: ptr :: null_mut () , std :: ptr :: null_mut ())) ; assert_eq ! (E_POINTER , ValidateJson (handle , std :: ptr :: null () , value . len () , std :: ptr :: null_mut () , std :: ptr :: null_mut ())) ; let value = r#""Hello"# ; let code = ValidateJson (handle , value . as_ptr () , value . len () , std :: ptr :: null_mut () , std :: ptr :: null_mut () ,) ; assert_eq ! (E_INVALIDARG , code) ; assert_eq ! ("EOF while parsing a string at line 1 column 6" , Error :: from (code) . message ()) ; CloseJsonValidator (handle) ; } }
};
}

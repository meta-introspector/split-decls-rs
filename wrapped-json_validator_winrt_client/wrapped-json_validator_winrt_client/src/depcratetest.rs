// Generated macro for test (function)
macro_rules! Depcratetest {
() => {
// Module: crate
// Provides: {"test"}
// Dependencies: {}
# [test] fn test () -> Result < () > { let validator = JsonValidator :: CreateInstance (h ! (r#"{"maxLength": 5}"#)) ? ; validator . Validate (h ! (r#""Hello""#)) ? ; let error = validator . Validate (h ! (r#""Hello World""#)) . unwrap_err () ; assert_eq ! (error . code () , E_INVALIDARG) ; assert_eq ! (error . message () , r#""Hello World" is longer than 5 characters"# ,) ; let error = JsonValidator :: CreateInstance (h ! (r#"{ "invalid"#)) . unwrap_err () ; assert_eq ! (error . code () , E_INVALIDARG) ; assert_eq ! (error . message () , "EOF while parsing a string at line 1 column 10" ,) ; let error = validator . Validate (h ! (r#""Hello"#)) . unwrap_err () ; assert_eq ! (error . code () , E_INVALIDARG) ; assert_eq ! (error . message () , "EOF while parsing a string at line 1 column 6" ,) ; Ok (()) }
};
}

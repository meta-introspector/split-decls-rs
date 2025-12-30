// Generated macro for validate_name (function)
macro_rules! Depcrate_rawvalidate_name {
() => {
// Module: crate::raw
// Provides: {"validate_name"}
// Dependencies: {}
fn validate_name (name : & [u8]) { assert ! (! name . is_empty () , "empty byte string") ; assert_eq ! (* name . last () . unwrap () , b'\0' , "non-null terminated byte string") ; }
};
}

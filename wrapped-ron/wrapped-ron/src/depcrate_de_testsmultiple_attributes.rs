// Generated macro for multiple_attributes (function)
macro_rules! Depcrate_de_testsmultiple_attributes {
() => {
// Module: crate::de::tests
// Provides: {"multiple_attributes"}
// Dependencies: {}
# [test] fn multiple_attributes () { # [derive (Debug , Deserialize , PartialEq)] struct New (String) ; check_from_str_bytes_reader ("#![enable(unwrap_newtypes)] #![enable(unwrap_newtypes)] \"Hello\"" , Ok (New ("Hello" . to_owned ())) ,) ; }
};
}

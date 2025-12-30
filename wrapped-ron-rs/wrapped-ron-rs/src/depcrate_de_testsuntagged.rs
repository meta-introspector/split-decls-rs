// Generated macro for untagged (function)
macro_rules! Depcrate_de_testsuntagged {
() => {
// Module: crate::de::tests
// Provides: {"untagged"}
// Dependencies: {}
# [test] fn untagged () { # [derive (Deserialize , Clone , Debug , PartialEq)] # [serde (untagged)] enum Untagged { U8 (u8) , Bool (bool) , Value (crate :: Value) , } check_from_str_bytes_reader ("true" , Ok (Untagged :: Bool (true))) ; check_from_str_bytes_reader ("8" , Ok (Untagged :: U8 (8))) ; let bogus_struct = "Value(()" ; let expected_err = Err (crate :: error :: SpannedError { code : crate :: Error :: Eof , span : Span { start : Position { line : 1 , col : 8 } , end : crate :: error :: Position { line : 1 , col : 9 } , } , }) ; check_from_str_bytes_reader :: < Untagged > (bogus_struct , expected_err . clone ()) ; # [cfg (feature = "internal-span-substring-test")] check_error_span_exclusive :: < Untagged > (bogus_struct , expected_err , ")") ; }
};
}

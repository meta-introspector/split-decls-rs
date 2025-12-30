// Generated macro for test_plus_operator (function)
macro_rules! Depcrate_teststest_plus_operator {
() => {
// Module: crate::tests
// Provides: {"test_plus_operator"}
// Dependencies: {}
# [allow (clippy :: unnecessary_to_owned , clippy :: op_ref)] # [test] fn test_plus_operator () { assert_eq ! (CompactString :: from ("a") + & CompactString :: from ("b") , "ab") ; assert_eq ! (CompactString :: from ("a") + "b" , "ab") ; assert_eq ! (CompactString :: from ("a") + & String :: from ("b") , "ab") ; let box_str = String :: from ("b") . into_boxed_str () ; assert_eq ! (CompactString :: from ("a") + & box_str , "ab") ; let cow = Cow :: from ("b") ; assert_eq ! (CompactString :: from ("a") + & cow , "ab") ; assert_eq ! (String :: from ("a") + & CompactString :: from ("b") , "ab") ; assert_eq ! (String :: from ("a") + & ("b" . to_string ()) , "ab") ; assert_eq ! (String :: from ("a") + "b" , "ab") ; }
};
}

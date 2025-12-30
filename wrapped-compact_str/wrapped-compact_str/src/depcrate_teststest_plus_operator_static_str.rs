// Generated macro for test_plus_operator_static_str (function)
macro_rules! Depcrate_teststest_plus_operator_static_str {
() => {
// Module: crate::tests
// Provides: {"test_plus_operator_static_str"}
// Dependencies: {}
# [allow (clippy :: unnecessary_to_owned , clippy :: op_ref)] # [test] fn test_plus_operator_static_str () { assert_eq ! (CompactString :: const_new ("a") + & CompactString :: const_new ("b") , "ab") ; assert_eq ! (CompactString :: const_new ("a") + "b" , "ab") ; assert_eq ! (CompactString :: const_new ("a") + & String :: from ("b") , "ab") ; let box_str = String :: from ("b") . into_boxed_str () ; assert_eq ! (CompactString :: const_new ("a") + & box_str , "ab") ; let cow = Cow :: from ("b") ; assert_eq ! (CompactString :: const_new ("a") + & cow , "ab") ; assert_eq ! (String :: from ("a") + & CompactString :: const_new ("b") , "ab") ; assert_eq ! (String :: from ("a") + & ("b" . to_string ()) , "ab") ; assert_eq ! (String :: from ("a") + "b" , "ab") ; }
};
}

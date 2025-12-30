// Generated macro for test_format_rust_expression (function)
macro_rules! Depcrate_utilstest_format_rust_expression {
() => {
// Module: crate::utils
// Provides: {"test_format_rust_expression"}
// Dependencies: {}
# [test] fn test_format_rust_expression () { use crate :: assert_snapshot ; assert_snapshot ! (format_rust_expression ("vec![1,2,3]") , @ "vec![1, 2, 3]") ; assert_snapshot ! (format_rust_expression ("vec![1,2,3].iter()") , @ "vec![1, 2, 3].iter()") ; assert_snapshot ! (format_rust_expression (r#"    "aoeu""#) , @ r#""aoeu""#) ; assert_snapshot ! (format_rust_expression (r#"  "aoe😄""#) , @ r#""aoe😄""#) ; assert_snapshot ! (format_rust_expression ("😄😄😄😄😄") , @ "😄😄😄😄😄") }
};
}

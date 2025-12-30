// Generated macro for test_to_string_num_keys (function)
macro_rules! Depcrate_content_jsontest_to_string_num_keys {
() => {
// Module: crate::content::json
// Provides: {"test_to_string_num_keys"}
// Dependencies: {}
# [test] fn test_to_string_num_keys () { let content = Content :: Map (vec ! [(Content :: from (42u32) , Content :: from (true)) , (Content :: from (- 23i32) , Content :: from (false)) ,]) ; let json = to_string_pretty (& content) ; crate :: assert_snapshot ! (& json , @ r#"
    {
      "42": true,
      "-23": false
    }
    "#) ; }
};
}

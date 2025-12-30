// Generated macro for test_debug (function)
macro_rules! Depcrate_tests_valuetest_debug {
() => {
// Module: crate::tests::value
// Provides: {"test_debug"}
// Dependencies: {}
# [test] fn test_debug () { let expected = [r#"NSValue { encoding: "C", bytes: (C) <ab> }"# , r#"NSValue { encoding: "C", bytes: {length = 1, bytes = 0xab} }"# , r#"NSValue { encoding: "C", bytes: <ab> }"# ,] ; let actual = format ! ("{:?}" , NSValue :: new (171u8)) ; assert ! (expected . contains (&&* actual) , "{actual}") ; }
};
}

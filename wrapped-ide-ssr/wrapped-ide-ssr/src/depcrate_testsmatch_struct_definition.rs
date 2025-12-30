// Generated macro for match_struct_definition (function)
macro_rules! Depcrate_testsmatch_struct_definition {
() => {
// Module: crate::tests
// Provides: {"match_struct_definition"}
// Dependencies: {}
# [test] fn match_struct_definition () { let code = r#"
        struct Option<T> {}
        struct Bar {}
        struct Foo {name: Option<String>}"# ; assert_matches ("struct $n {$f: Option<String>}" , code , & ["struct Foo {name: Option<String>}"]) ; }
};
}

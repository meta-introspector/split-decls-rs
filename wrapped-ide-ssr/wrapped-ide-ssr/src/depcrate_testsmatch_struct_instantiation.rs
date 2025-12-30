// Generated macro for match_struct_instantiation (function)
macro_rules! Depcrate_testsmatch_struct_instantiation {
() => {
// Module: crate::tests
// Provides: {"match_struct_instantiation"}
// Dependencies: {}
# [test] fn match_struct_instantiation () { let code = r#"
        struct Foo {bar: i32, baz: i32}
        fn f() {Foo {bar: 1, baz: 2}}"# ; assert_matches ("Foo {bar: 1, baz: 2}" , code , & ["Foo {bar: 1, baz: 2}"]) ; assert_matches ("Foo {$a: $b, $c: $d}" , code , & ["Foo {bar: 1, baz: 2}"]) ; assert_matches ("Foo {}" , "struct Foo {} fn f() {Foo {}}" , & ["Foo {}"]) ; }
};
}

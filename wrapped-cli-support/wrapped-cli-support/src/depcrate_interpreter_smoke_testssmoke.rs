// Generated macro for smoke (function)
macro_rules! Depcrate_interpreter_smoke_testssmoke {
() => {
// Module: crate::interpreter::smoke_tests
// Provides: {"smoke"}
// Dependencies: {}
# [test] fn smoke () { let wat = r#"
        (module
            (export "foo" (func $foo))

            (func $foo)
        )
    "# ; interpret (wat , "foo" , & []) ; let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))

            (func $foo
                i32.const 1
                call $__wbindgen_describe
            )

            (export "foo" (func $foo))
        )
    "# ; interpret (wat , "foo" , & [1]) ; }
};
}

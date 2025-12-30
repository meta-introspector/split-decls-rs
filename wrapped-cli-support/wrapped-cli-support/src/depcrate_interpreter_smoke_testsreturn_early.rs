// Generated macro for return_early (function)
macro_rules! Depcrate_interpreter_smoke_testsreturn_early {
() => {
// Module: crate::interpreter::smoke_tests
// Provides: {"return_early"}
// Dependencies: {}
# [test] fn return_early () { let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))

            (func $foo
                i32.const 1
                i32.const 2
                call $__wbindgen_describe
                return
            )

            (export "foo" (func $foo))
        )
    "# ; interpret (wat , "foo" , & [2]) ; }
};
}

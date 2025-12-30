// Generated macro for calling_functions (function)
macro_rules! Depcrate_interpreter_smoke_testscalling_functions {
() => {
// Module: crate::interpreter::smoke_tests
// Provides: {"calling_functions"}
// Dependencies: {}
# [test] fn calling_functions () { let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))

            (global i32 (i32.const 0))
            (memory 1)

            (func $foo
                call $bar
            )

            (func $bar
                i32.const 0
                call $__wbindgen_describe
            )

            (export "foo" (func $foo))
        )
    "# ; interpret (wat , "foo" , & [0]) ; }
};
}

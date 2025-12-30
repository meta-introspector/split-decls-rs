// Generated macro for arithmetic (function)
macro_rules! Depcrate_interpreter_smoke_testsarithmetic {
() => {
// Module: crate::interpreter::smoke_tests
// Provides: {"arithmetic"}
// Dependencies: {}
# [test] fn arithmetic () { let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))

            (func $foo
                i32.const 1
                i32.const 2
                i32.add
                call $__wbindgen_describe
                i32.const 2
                i32.const 1
                i32.sub
                call $__wbindgen_describe
            )

            (export "foo" (func $foo))
        )
    "# ; interpret (wat , "foo" , & [3 , 1]) ; }
};
}

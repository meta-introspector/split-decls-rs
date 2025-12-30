// Generated macro for locals (function)
macro_rules! Depcrate_interpreter_smoke_testslocals {
() => {
// Module: crate::interpreter::smoke_tests
// Provides: {"locals"}
// Dependencies: {}
# [test] fn locals () { let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))

            (func $foo
                (local i32)
                i32.const 2
                local.set 0
                local.get 0
                call $__wbindgen_describe
            )

            (export "foo" (func $foo))
        )
    "# ; interpret (wat , "foo" , & [2]) ; }
};
}

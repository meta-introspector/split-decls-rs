// Generated macro for globals (function)
macro_rules! Depcrate_interpreter_smoke_testsglobals {
() => {
// Module: crate::interpreter::smoke_tests
// Provides: {"globals"}
// Dependencies: {}
# [test] fn globals () { let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))

            (global (mut i32) (i32.const 0))

            (func $foo
                (local i32)
                global.get 0
                local.set 0
                local.get 0
                call $__wbindgen_describe
                local.get 0
                global.set 0
            )

            (export "foo" (func $foo))
        )
    "# ; interpret (wat , "foo" , & [32768]) ; }
};
}

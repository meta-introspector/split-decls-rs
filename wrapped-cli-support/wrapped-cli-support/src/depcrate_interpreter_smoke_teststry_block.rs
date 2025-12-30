// Generated macro for try_block (function)
macro_rules! Depcrate_interpreter_smoke_teststry_block {
() => {
// Module: crate::interpreter::smoke_tests
// Provides: {"try_block"}
// Dependencies: {}
# [test] fn try_block () { let wat = r#"
        (module
            (export "foo" (func $foo))

            (func $foo)
        )
    "# ; interpret (wat , "foo" , & []) ; let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))
            (global (mut i32) (i32.const 0))

            (func $foo
                (local i32)

                ;; decrement the stack pointer, setting our local to the
                ;; lowest address of our stack
                global.get 0
                i32.const 16
                i32.sub
                local.set 0
                local.get 0
                global.set 0

                try
                    i32.const 1
                    call $__wbindgen_describe
                catch_all
                end

                ;; increment our stack pointer
                local.get 0
                i32.const 16
                i32.add
                global.set 0
            )

            (export "foo" (func $foo))
        )
    "# ; interpret (wat , "foo" , & [1]) ; }
};
}

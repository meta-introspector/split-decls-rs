// Generated macro for loads_and_stores (function)
macro_rules! Depcrate_interpreter_smoke_testsloads_and_stores {
() => {
// Module: crate::interpreter::smoke_tests
// Provides: {"loads_and_stores"}
// Dependencies: {}
# [test] fn loads_and_stores () { let wat = r#"
        (module
            (import "__wbindgen_placeholder__" "__wbindgen_describe"
              (func $__wbindgen_describe (param i32)))

            (global (mut i32) (i32.const 0))
            (memory 1)

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

                ;; store 1 at fp+0
                local.get 0
                i32.const 1
                i32.store offset=0

                ;; store 2 at fp+4
                local.get 0
                i32.const 2
                i32.store offset=4

                ;; store 3 at fp+8
                local.get 0
                i32.const 3
                i32.store offset=8

                ;; store8
                local.get 0
                i32.const 3
                i32.store8 offset=7

                ;; load8
                local.get 0
                i32.load8_u offset=7
                drop

                ;; load fp+0 and call
                local.get 0
                i32.load offset=0
                call $__wbindgen_describe

                ;; load fp+4 and call
                local.get 0
                i32.load offset=4
                call $__wbindgen_describe

                ;; load fp+8 and call
                local.get 0
                i32.load offset=8
                call $__wbindgen_describe

                ;; increment our stack pointer
                local.get 0
                i32.const 16
                i32.add
                global.set 0
            )

            (export "foo" (func $foo))
        )
    "# ; interpret (wat , "foo" , & [1 , 50331650 , 3]) ; }
};
}

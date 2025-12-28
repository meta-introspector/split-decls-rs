macro_rules! panic_propagate_still_execute_3 {
    () => {
        # [test] # [cfg_attr (not (panic = "unwind") , ignore)] fn panic_propagate_still_execute_3 () { let mut x = false ; let result = unwind :: halt_unwinding (| | { scope (| s | { s . spawn (| _ | x = true) ; panic ! ("Hello, world!") ; }) ; }) ; match result { Ok (_) => panic ! ("failed to propagate panic") , Err (_) => assert ! (x , "panic after spawn, spawn failed to execute") , } }
    };
}

panic_propagate_still_execute_3!();
// Generated macro for panic_propagate_still_execute_2 (function)
macro_rules! Depcrate_scope_testpanic_propagate_still_execute_2 {
() => {
// Module: crate::scope::test
// Provides: {"panic_propagate_still_execute_2"}
// Dependencies: {}
# [test] # [cfg_attr (not (panic = "unwind") , ignore)] fn panic_propagate_still_execute_2 () { let mut x = false ; let result = unwind :: halt_unwinding (| | { scope (| s | { s . spawn (| _ | x = true) ; s . spawn (| _ | panic ! ("Hello, world!")) ; }) ; }) ; match result { Ok (_) => panic ! ("failed to propagate panic") , Err (_) => assert ! (x , "job b failed to execute") , } }
};
}

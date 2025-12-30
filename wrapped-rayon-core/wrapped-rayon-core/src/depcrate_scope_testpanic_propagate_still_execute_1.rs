// Generated macro for panic_propagate_still_execute_1 (function)
macro_rules! Depcrate_scope_testpanic_propagate_still_execute_1 {
() => {
// Module: crate::scope::test
// Provides: {"panic_propagate_still_execute_1"}
// Dependencies: {}
# [test] # [cfg_attr (not (panic = "unwind") , ignore)] fn panic_propagate_still_execute_1 () { let mut x = false ; let result = unwind :: halt_unwinding (| | { scope (| s | { s . spawn (| _ | panic ! ("Hello, world!")) ; s . spawn (| _ | x = true) ; }) ; }) ; match result { Ok (_) => panic ! ("failed to propagate panic") , Err (_) => assert ! (x , "job b failed to execute") , } }
};
}

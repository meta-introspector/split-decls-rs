// Generated macro for bug_on_incorrect_arg_count (function)
macro_rules! Depcrate_intrinsicsbug_on_incorrect_arg_count {
() => {
// Module: crate::intrinsics
// Provides: {"bug_on_incorrect_arg_count"}
// Dependencies: {}
fn bug_on_incorrect_arg_count (intrinsic : impl std :: fmt :: Display) -> ! { bug ! ("wrong number of args for intrinsic {}" , intrinsic) ; }
};
}

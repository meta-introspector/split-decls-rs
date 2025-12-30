// Generated macro for macro_348 (macro)
macro_rules! Depcrate_arm_linuxmacro_348 {
() => {
// Module: crate::arm_linux
// Provides: {"macro_348"}
// Dependencies: {}
atomic_rmw ! (@ old __sync_fetch_and_max_4 , i32 , | a : i32 , b : i32 | if a > b { a } else { b }) ;
};
}

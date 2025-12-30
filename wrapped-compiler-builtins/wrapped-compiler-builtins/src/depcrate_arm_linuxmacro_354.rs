// Generated macro for macro_354 (macro)
macro_rules! Depcrate_arm_linuxmacro_354 {
() => {
// Module: crate::arm_linux
// Provides: {"macro_354"}
// Dependencies: {}
atomic_rmw ! (@ old __sync_fetch_and_min_4 , i32 , | a : i32 , b : i32 | if a < b { a } else { b }) ;
};
}

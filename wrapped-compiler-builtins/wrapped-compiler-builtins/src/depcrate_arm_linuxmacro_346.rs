// Generated macro for macro_346 (macro)
macro_rules! Depcrate_arm_linuxmacro_346 {
() => {
// Module: crate::arm_linux
// Provides: {"macro_346"}
// Dependencies: {}
atomic_rmw ! (@ old __sync_fetch_and_max_1 , i8 , | a : i8 , b : i8 | if a > b { a } else { b }) ;
};
}

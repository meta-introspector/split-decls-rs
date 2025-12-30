// Generated macro for macro_352 (macro)
macro_rules! Depcrate_arm_linuxmacro_352 {
() => {
// Module: crate::arm_linux
// Provides: {"macro_352"}
// Dependencies: {}
atomic_rmw ! (@ old __sync_fetch_and_min_1 , i8 , | a : i8 , b : i8 | if a < b { a } else { b }) ;
};
}

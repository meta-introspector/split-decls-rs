// Generated macro for macro_353 (macro)
macro_rules! Depcrate_arm_linuxmacro_353 {
() => {
// Module: crate::arm_linux
// Provides: {"macro_353"}
// Dependencies: {}
atomic_rmw ! (@ old __sync_fetch_and_min_2 , i16 , | a : i16 , b : i16 | if a < b { a } else { b }) ;
};
}

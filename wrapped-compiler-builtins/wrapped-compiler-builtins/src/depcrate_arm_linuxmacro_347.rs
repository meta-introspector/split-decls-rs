// Generated macro for macro_347 (macro)
macro_rules! Depcrate_arm_linuxmacro_347 {
() => {
// Module: crate::arm_linux
// Provides: {"macro_347"}
// Dependencies: {}
atomic_rmw ! (@ old __sync_fetch_and_max_2 , i16 , | a : i16 , b : i16 | if a > b { a } else { b }) ;
};
}

// Generated macro for macro_349 (macro)
macro_rules! Depcrate_arm_linuxmacro_349 {
() => {
// Module: crate::arm_linux
// Provides: {"macro_349"}
// Dependencies: {}
atomic_rmw ! (@ old __sync_fetch_and_umax_1 , u8 , | a : u8 , b : u8 | if a > b { a } else { b }) ;
};
}

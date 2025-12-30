// Generated macro for macro_351 (macro)
macro_rules! Depcrate_arm_linuxmacro_351 {
() => {
// Module: crate::arm_linux
// Provides: {"macro_351"}
// Dependencies: {}
atomic_rmw ! (@ old __sync_fetch_and_umax_4 , u32 , | a : u32 , b : u32 | if a > b { a } else { b }) ;
};
}

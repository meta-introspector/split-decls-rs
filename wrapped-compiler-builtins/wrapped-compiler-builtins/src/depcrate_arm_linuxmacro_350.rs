// Generated macro for macro_350 (macro)
macro_rules! Depcrate_arm_linuxmacro_350 {
() => {
// Module: crate::arm_linux
// Provides: {"macro_350"}
// Dependencies: {}
atomic_rmw ! (@ old __sync_fetch_and_umax_2 , u16 , | a : u16 , b : u16 | if a > b { a } else { b }) ;
};
}

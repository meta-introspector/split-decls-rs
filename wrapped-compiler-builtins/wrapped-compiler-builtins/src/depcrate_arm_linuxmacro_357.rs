// Generated macro for macro_357 (macro)
macro_rules! Depcrate_arm_linuxmacro_357 {
() => {
// Module: crate::arm_linux
// Provides: {"macro_357"}
// Dependencies: {}
atomic_rmw ! (@ old __sync_fetch_and_umin_4 , u32 , | a : u32 , b : u32 | if a < b { a } else { b }) ;
};
}

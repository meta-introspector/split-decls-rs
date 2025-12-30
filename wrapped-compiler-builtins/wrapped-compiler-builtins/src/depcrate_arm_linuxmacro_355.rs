// Generated macro for macro_355 (macro)
macro_rules! Depcrate_arm_linuxmacro_355 {
() => {
// Module: crate::arm_linux
// Provides: {"macro_355"}
// Dependencies: {}
atomic_rmw ! (@ old __sync_fetch_and_umin_1 , u8 , | a : u8 , b : u8 | if a < b { a } else { b }) ;
};
}

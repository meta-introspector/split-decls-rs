// Generated macro for macro_356 (macro)
macro_rules! Depcrate_arm_linuxmacro_356 {
() => {
// Module: crate::arm_linux
// Provides: {"macro_356"}
// Dependencies: {}
atomic_rmw ! (@ old __sync_fetch_and_umin_2 , u16 , | a : u16 , b : u16 | if a < b { a } else { b }) ;
};
}

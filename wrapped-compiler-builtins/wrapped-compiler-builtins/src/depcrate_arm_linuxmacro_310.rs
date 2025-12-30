// Generated macro for macro_310 (macro)
macro_rules! Depcrate_arm_linuxmacro_310 {
() => {
// Module: crate::arm_linux
// Provides: {"macro_310"}
// Dependencies: {}
atomic_rmw ! (@ old __sync_fetch_and_add_1 , u8 , | a : u8 , b : u8 | a . wrapping_add (b)) ;
};
}

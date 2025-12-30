// Generated macro for z_off_t (type)
macro_rules! Depcratez_off_t {
() => {
// Module: crate
// Provides: {"z_off_t"}
// Dependencies: {}
# [cfg (all (zng , not (all (windows , not (target_env = "gnu")))))] pub type z_off_t = libc :: off_t ;
};
}

// Generated macro for sol_remaining_compute_units (function)
macro_rules! Depcrate_compute_unitssol_remaining_compute_units {
() => {
// Module: crate::compute_units
// Provides: {"sol_remaining_compute_units"}
// Dependencies: {}
# [doc = " Return the remaining compute units the program may consume"] # [inline] pub fn sol_remaining_compute_units () -> u64 { # [cfg (target_os = "solana")] unsafe { crate :: syscalls :: sol_remaining_compute_units () } # [cfg (not (target_os = "solana"))] { crate :: program_stubs :: sol_remaining_compute_units () } }
};
}

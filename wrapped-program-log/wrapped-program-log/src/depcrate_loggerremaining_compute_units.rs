// Generated macro for remaining_compute_units (function)
macro_rules! Depcrate_loggerremaining_compute_units {
() => {
// Module: crate::logger
// Provides: {"remaining_compute_units"}
// Dependencies: {}
# [doc = " Remaining CUs."] # [inline (always)] pub fn remaining_compute_units () -> u64 { # [cfg (any (target_os = "solana" , target_arch = "bpf"))] unsafe { sol_remaining_compute_units () } # [cfg (not (any (target_os = "solana" , target_arch = "bpf")))] core :: hint :: black_box (0u64) }
};
}

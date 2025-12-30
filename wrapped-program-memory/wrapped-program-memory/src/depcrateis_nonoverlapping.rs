// Generated macro for is_nonoverlapping (function)
macro_rules! Depcrateis_nonoverlapping {
() => {
// Module: crate
// Provides: {"is_nonoverlapping"}
// Dependencies: {}
# [doc = " Check that two regions do not overlap."] # [cfg (any (test , not (any (target_os = "solana" , target_arch = "bpf"))))] fn is_nonoverlapping (src : usize , src_len : usize , dst : usize , dst_len : usize) -> bool { if src > dst { src . saturating_sub (dst) >= dst_len } else { dst . saturating_sub (src) >= src_len } }
};
}

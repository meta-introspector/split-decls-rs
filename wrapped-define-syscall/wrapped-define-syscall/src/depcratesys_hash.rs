// Generated macro for sys_hash (function)
macro_rules! Depcratesys_hash {
() => {
// Module: crate
// Provides: {"sys_hash"}
// Dependencies: {}
# [cfg (any (target_feature = "static-syscalls" , all (target_arch = "bpf" , feature = "unstable-static-syscalls")))] pub const fn sys_hash (name : & str) -> usize { murmur3_32 (name . as_bytes () , 0) as usize }
};
}

// Generated macro for use_24 (pub_use)
macro_rules! Depcrate_syscallsuse_24 {
() => {
// Module: crate::syscalls
// Provides: {"use_24"}
// Dependencies: {}
# [doc = " Syscall definitions used by `solana_address`."] # [cfg (any (target_os = "solana" , target_arch = "bpf"))] pub use solana_define_syscall :: definitions :: { sol_create_program_address , sol_log_pubkey , sol_try_find_program_address , } ;
};
}

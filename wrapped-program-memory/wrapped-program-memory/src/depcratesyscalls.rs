// Generated macro for syscalls (module)
macro_rules! Depcratesyscalls {
() => {
// Module: crate
// Provides: {"syscalls"}
// Dependencies: {}
# [cfg (any (target_os = "solana" , target_arch = "bpf"))] pub mod syscalls { pub use solana_define_syscall :: definitions :: { sol_memcmp_ , sol_memcpy_ , sol_memmove_ , sol_memset_ , } ; }
};
}

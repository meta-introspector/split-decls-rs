// Generated macro for use_4 (pub_use)
macro_rules! Depcrate_cpiuse_4 {
() => {
// Module: crate::cpi
// Provides: {"use_4"}
// Dependencies: {}
# [cfg (any (target_os = "solana" , target_arch = "bpf"))] pub use solana_define_syscall :: { define_syscall , definitions :: { sol_get_return_data , sol_invoke_signed_c , sol_set_return_data } , } ;
};
}

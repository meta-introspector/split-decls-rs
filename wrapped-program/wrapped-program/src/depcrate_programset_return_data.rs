// Generated macro for set_return_data (function)
macro_rules! Depcrate_programset_return_data {
() => {
// Module: crate::program
// Provides: {"set_return_data"}
// Dependencies: {}
# [doc = " Like [`solana_cpi::set_return_data`], but with support"] # [doc = " for overwriting the `sol_set_return_data` syscall stub."] # [doc = ""] # [doc = " [`solana_cpi::set_return_data`]: https://docs.rs/solana-cpi/latest/solana_cpi/fn.set_return_data.html"] pub fn set_return_data (data : & [u8]) { # [cfg (target_os = "solana")] { solana_cpi :: set_return_data (data) ; } # [cfg (not (target_os = "solana"))] crate :: program_stubs :: sol_set_return_data (data) }
};
}

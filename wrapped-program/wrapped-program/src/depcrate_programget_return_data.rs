// Generated macro for get_return_data (function)
macro_rules! Depcrate_programget_return_data {
() => {
// Module: crate::program
// Provides: {"get_return_data"}
// Dependencies: {}
# [doc = " Like [`solana_cpi::get_return_data`], but with support"] # [doc = " for overwriting the `sol_get_return_data` syscall stub."] # [doc = ""] # [doc = " [`solana_cpi::get_return_data`]: https://docs.rs/solana-cpi/latest/solana_cpi/fn.get_return_data.html"] pub fn get_return_data () -> Option < (Pubkey , Vec < u8 >) > { # [cfg (target_os = "solana")] { solana_cpi :: get_return_data () } # [cfg (not (target_os = "solana"))] crate :: program_stubs :: sol_get_return_data () }
};
}

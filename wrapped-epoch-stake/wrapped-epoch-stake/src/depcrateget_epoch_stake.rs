// Generated macro for get_epoch_stake (function)
macro_rules! Depcrateget_epoch_stake {
() => {
// Module: crate
// Provides: {"get_epoch_stake"}
// Dependencies: {}
fn get_epoch_stake (var_addr : * const u8) -> u64 { # [cfg (target_os = "solana")] { unsafe { solana_define_syscall :: definitions :: sol_get_epoch_stake (var_addr) } } # [cfg (not (target_os = "solana"))] { core :: hint :: black_box (var_addr) ; 0 } }
};
}

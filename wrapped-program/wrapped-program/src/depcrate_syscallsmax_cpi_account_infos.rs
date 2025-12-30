// Generated macro for MAX_CPI_ACCOUNT_INFOS (const)
macro_rules! Depcrate_syscallsMAX_CPI_ACCOUNT_INFOS {
() => {
// Module: crate::syscalls
// Provides: {"MAX_CPI_ACCOUNT_INFOS"}
// Dependencies: {}
# [doc = " Maximum number of account info structs that can be used in a single CPI"] # [doc = " invocation. A limit on account info structs is effectively the same as"] # [doc = " limiting the number of unique accounts. 128 was chosen to match the max"] # [doc = " number of locked accounts per transaction (MAX_TX_ACCOUNT_LOCKS)."] pub const MAX_CPI_ACCOUNT_INFOS : usize = 128 ;
};
}

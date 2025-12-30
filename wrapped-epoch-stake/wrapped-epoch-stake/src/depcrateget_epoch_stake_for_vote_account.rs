// Generated macro for get_epoch_stake_for_vote_account (function)
macro_rules! Depcrateget_epoch_stake_for_vote_account {
() => {
// Module: crate
// Provides: {"get_epoch_stake_for_vote_account"}
// Dependencies: {}
# [doc = " Get the current epoch stake for a given vote address."] # [doc = ""] # [doc = " If the provided vote address corresponds to an account that is not a vote"] # [doc = " account or does not exist, returns `0` for active stake."] pub fn get_epoch_stake_for_vote_account (vote_address : & Pubkey) -> u64 { get_epoch_stake (vote_address as * const _ as * const u8) }
};
}

// Generated macro for use_77 (pub_use)
macro_rules! Depcrate_sysvaruse_77 {
() => {
// Module: crate::sysvar
// Provides: {"use_77"}
// Dependencies: {}
# [deprecated (since = "2.2.0" , note = "Use `solana-sysvar` crate instead")] # [allow (deprecated)] pub use { solana_sdk_ids :: sysvar :: { check_id , id , ID } , solana_sysvar :: { clock , epoch_rewards , epoch_schedule , fees , last_restart_slot , recent_blockhashes , rent , rewards , slot_hashes , slot_history , Sysvar , SysvarSerialize , } , } ;
};
}

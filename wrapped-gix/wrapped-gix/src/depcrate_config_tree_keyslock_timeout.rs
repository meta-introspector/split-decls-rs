// Generated macro for lock_timeout (module)
macro_rules! Depcrate_config_tree_keyslock_timeout {
() => {
// Module: crate::config::tree::keys
// Provides: {"lock_timeout"}
// Dependencies: {}
mod lock_timeout { use std :: time :: Duration ; use gix_lock :: acquire :: Fail ; use crate :: { config , config :: tree :: { keys :: LockTimeout , Section } , } ; impl LockTimeout { # [doc = " Create a new instance."] pub const fn new_lock_timeout (name : & 'static str , section : & 'static dyn Section) -> Self { Self :: new_with_validate (name , section , super :: validate :: LockTimeout) } # [doc = " Return information on how long to wait for locked files."] pub fn try_into_lock_timeout (& 'static self , value : Result < i64 , gix_config :: value :: Error > ,) -> Result < gix_lock :: acquire :: Fail , config :: lock_timeout :: Error > { let value = value . map_err (| err | config :: lock_timeout :: Error :: from (self) . with_source (err)) ? ; Ok (match value { val if val < 0 => Fail :: AfterDurationWithBackoff (Duration :: from_secs (u64 :: MAX)) , 0 => Fail :: Immediately , val => Fail :: AfterDurationWithBackoff (Duration :: from_millis (val . try_into () . expect ("i64 to u64 always works if positive") ,)) , }) } } }
};
}

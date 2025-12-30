// Generated macro for duration (module)
macro_rules! Depcrate_config_tree_keysduration {
() => {
// Module: crate::config::tree::keys
// Provides: {"duration"}
// Dependencies: {}
mod duration { use std :: time :: Duration ; use crate :: { config , config :: tree :: { keys :: DurationInMilliseconds , Section } , } ; impl DurationInMilliseconds { # [doc = " Create a new instance."] pub const fn new_duration (name : & 'static str , section : & 'static dyn Section) -> Self { Self :: new_with_validate (name , section , super :: validate :: DurationInMilliseconds) } # [doc = " Return a valid duration as parsed from an integer that is interpreted as milliseconds."] pub fn try_into_duration (& 'static self , value : Result < i64 , gix_config :: value :: Error > ,) -> Result < std :: time :: Duration , config :: duration :: Error > { let value = value . map_err (| err | config :: duration :: Error :: from (self) . with_source (err)) ? ; Ok (match value { val if val < 0 => Duration :: from_secs (u64 :: MAX) , val => Duration :: from_millis (val . try_into () . expect ("i64 to u64 always works if positive")) , }) } } }
};
}

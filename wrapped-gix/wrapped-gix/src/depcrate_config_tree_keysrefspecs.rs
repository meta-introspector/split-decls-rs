// Generated macro for refspecs (module)
macro_rules! Depcrate_config_tree_keysrefspecs {
() => {
// Module: crate::config::tree::keys
// Provides: {"refspecs"}
// Dependencies: {}
mod refspecs { use crate :: config :: tree :: { keys :: { validate , FetchRefSpec , PushRefSpec } , Section , } ; impl PushRefSpec { # [doc = " Create a new instance."] pub const fn new_push_refspec (name : & 'static str , section : & 'static dyn Section) -> Self { Self :: new_with_validate (name , section , validate :: PushRefSpec) } } impl FetchRefSpec { # [doc = " Create a new instance."] pub const fn new_fetch_refspec (name : & 'static str , section : & 'static dyn Section) -> Self { Self :: new_with_validate (name , section , validate :: FetchRefSpec) } } }
};
}

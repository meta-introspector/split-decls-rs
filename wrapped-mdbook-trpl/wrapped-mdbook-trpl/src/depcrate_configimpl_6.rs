// Generated macro for impl_6 (impl)
macro_rules! Depcrate_configimpl_6 {
() => {
// Module: crate::config
// Provides: {"impl_6"}
// Dependencies: {}
impl TryFrom < & str > for Mode { type Error = ParseErr ; fn try_from (value : & str) -> Result < Self , Self :: Error > { match value { "default" => Ok (Mode :: Default) , "simple" => Ok (Mode :: Simple) , _ => Err (ParseErr) , } } }
};
}

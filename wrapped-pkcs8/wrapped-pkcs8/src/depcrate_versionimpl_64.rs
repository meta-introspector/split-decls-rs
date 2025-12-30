// Generated macro for impl_64 (impl)
macro_rules! Depcrate_versionimpl_64 {
() => {
// Module: crate::version
// Provides: {"impl_64"}
// Dependencies: {}
impl TryFrom < u8 > for Version { type Error = Error ; fn try_from (byte : u8) -> Result < Version , Error > { match byte { 0 => Ok (Version :: V1) , 1 => Ok (Version :: V2) , _ => Err (Self :: TAG . value_error () . into ()) , } } }
};
}

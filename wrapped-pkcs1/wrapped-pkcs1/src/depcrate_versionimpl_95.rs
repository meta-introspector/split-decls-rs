// Generated macro for impl_95 (impl)
macro_rules! Depcrate_versionimpl_95 {
() => {
// Module: crate::version
// Provides: {"impl_95"}
// Dependencies: {}
impl TryFrom < u8 > for Version { type Error = Error ; fn try_from (byte : u8) -> Result < Version , Error > { match byte { 0 => Ok (Version :: TwoPrime) , 1 => Ok (Version :: Multi) , _ => Err (Error :: Version) , } } }
};
}

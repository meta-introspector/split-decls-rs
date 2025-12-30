// Generated macro for impl_12 (impl)
macro_rules! Depcrate_configimpl_12 {
() => {
// Module: crate::config
// Provides: {"impl_12"}
// Dependencies: {}
impl TryFrom < & BStr > for Ignore { type Error = () ; fn try_from (value : & BStr) -> Result < Self , Self :: Error > { Ok (match value . as_bytes () { b"all" => Ignore :: All , b"dirty" => Ignore :: Dirty , b"untracked" => Ignore :: Untracked , b"none" => Ignore :: None , _ => return Err (()) , }) } }
};
}

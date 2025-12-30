// Generated macro for impl_19 (impl)
macro_rules! Depcrate_configimpl_19 {
() => {
// Module: crate::config
// Provides: {"impl_19"}
// Dependencies: {}
impl TryFrom < & BStr > for Update { type Error = () ; fn try_from (value : & BStr) -> Result < Self , Self :: Error > { Ok (match value . as_bstr () . as_bytes () { b"checkout" => Update :: Checkout , b"rebase" => Update :: Rebase , b"merge" => Update :: Merge , b"none" => Update :: None , command if command . first () == Some (& b'!') => Update :: Command (command [1 ..] . to_owned () . into ()) , _ => return Err (()) , }) } }
};
}

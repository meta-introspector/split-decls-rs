// Generated macro for impl_226 (impl)
macro_rules! Depcrateimpl_226 {
() => {
// Module: crate
// Provides: {"impl_226"}
// Dependencies: {}
impl TryFrom < Value > for String { type Error = Value ; fn try_from (val : Value) -> Result < Self , Self :: Error > { match val { Value :: String (Utf8String { s : Ok (u) }) => Ok (u) , _ => Err (val) , } } }
};
}

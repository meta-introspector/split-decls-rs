// Generated macro for impl_90 (impl)
macro_rules! Depcrate_valueimpl_90 {
() => {
// Module: crate::value
// Provides: {"impl_90"}
// Dependencies: {}
impl TryFrom < Value > for HSTRING { type Error = Error ; fn try_from (from : Value) -> Result < Self > { match from . ty { Type :: String | Type :: ExpandString => Ok (Self :: from_wide (trim (from . data . as_wide ()))) , _ => Err (invalid_data ()) , } } }
};
}

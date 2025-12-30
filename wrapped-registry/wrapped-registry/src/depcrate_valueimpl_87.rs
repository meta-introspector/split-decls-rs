// Generated macro for impl_87 (impl)
macro_rules! Depcrate_valueimpl_87 {
() => {
// Module: crate::value
// Provides: {"impl_87"}
// Dependencies: {}
impl TryFrom < Value > for String { type Error = Error ; fn try_from (from : Value) -> Result < Self > { match from . ty { Type :: String | Type :: ExpandString => Ok (Self :: from_utf16 (trim (from . data . as_wide ())) ?) , _ => Err (invalid_data ()) , } } }
};
}

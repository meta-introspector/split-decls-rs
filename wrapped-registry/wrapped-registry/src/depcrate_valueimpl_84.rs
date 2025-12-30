// Generated macro for impl_84 (impl)
macro_rules! Depcrate_valueimpl_84 {
() => {
// Module: crate::value
// Provides: {"impl_84"}
// Dependencies: {}
impl TryFrom < Value > for u32 { type Error = Error ; fn try_from (from : Value) -> Result < Self > { Ok (from_le_bytes (from . ty , & from) ? . try_into () ?) } }
};
}

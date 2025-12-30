// Generated macro for impl_86 (impl)
macro_rules! Depcrate_valueimpl_86 {
() => {
// Module: crate::value
// Provides: {"impl_86"}
// Dependencies: {}
impl TryFrom < Value > for u64 { type Error = Error ; fn try_from (from : Value) -> Result < Self > { from_le_bytes (from . ty , & from) } }
};
}

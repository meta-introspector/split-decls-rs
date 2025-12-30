// Generated macro for impl_83 (impl)
macro_rules! Depcrate_valueimpl_83 {
() => {
// Module: crate::value
// Provides: {"impl_83"}
// Dependencies: {}
impl From < u32 > for Value { fn from (from : u32) -> Self { Self { data : from . to_le_bytes () . into () , ty : Type :: U32 , } } }
};
}

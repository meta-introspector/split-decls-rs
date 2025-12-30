// Generated macro for impl_85 (impl)
macro_rules! Depcrate_valueimpl_85 {
() => {
// Module: crate::value
// Provides: {"impl_85"}
// Dependencies: {}
impl From < u64 > for Value { fn from (from : u64) -> Self { Self { data : from . to_le_bytes () . into () , ty : Type :: U64 , } } }
};
}

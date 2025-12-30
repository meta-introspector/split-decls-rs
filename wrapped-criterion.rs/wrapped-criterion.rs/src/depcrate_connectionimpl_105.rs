// Generated macro for impl_105 (impl)
macro_rules! Depcrate_connectionimpl_105 {
() => {
// Module: crate::connection
// Provides: {"impl_105"}
// Dependencies: {}
impl From < ciborium :: ser :: Error < std :: io :: Error > > for MessageError { fn from (other : ciborium :: ser :: Error < std :: io :: Error >) -> Self { MessageError :: Serialization (other) } }
};
}

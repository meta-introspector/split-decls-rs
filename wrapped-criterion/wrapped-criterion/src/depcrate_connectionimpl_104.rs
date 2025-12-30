// Generated macro for impl_104 (impl)
macro_rules! Depcrate_connectionimpl_104 {
() => {
// Module: crate::connection
// Provides: {"impl_104"}
// Dependencies: {}
impl From < ciborium :: de :: Error < std :: io :: Error > > for MessageError { fn from (other : ciborium :: de :: Error < std :: io :: Error >) -> Self { MessageError :: Deserialization (other) } }
};
}

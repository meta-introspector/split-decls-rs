// Generated macro for impl_61 (impl)
macro_rules! Depcrateimpl_61 {
() => {
// Module: crate
// Provides: {"impl_61"}
// Dependencies: {}
impl < Tuple : Ord > std :: ops :: Deref for Relation < Tuple > { type Target = [Tuple] ; fn deref (& self) -> & Self :: Target { & self . elements [..] } }
};
}

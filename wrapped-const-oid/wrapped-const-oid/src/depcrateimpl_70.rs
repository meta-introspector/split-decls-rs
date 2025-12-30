// Generated macro for impl_70 (impl)
macro_rules! Depcrateimpl_70 {
() => {
// Module: crate
// Provides: {"impl_70"}
// Dependencies: {}
impl < const MAX_SIZE : usize > Deref for ObjectIdentifier < MAX_SIZE > { type Target = ObjectIdentifierRef ; fn deref (& self) -> & ObjectIdentifierRef { self . as_oid_ref () } }
};
}

// Generated macro for impl_84 (impl)
macro_rules! Depcrateimpl_84 {
() => {
// Module: crate
// Provides: {"impl_84"}
// Dependencies: {}
impl < const MAX_SIZE : usize > PartialEq < ObjectIdentifier < MAX_SIZE > > for ObjectIdentifierRef { fn eq (& self , other : & ObjectIdentifier < MAX_SIZE >) -> bool { self . as_bytes () . eq (other . as_bytes ()) } }
};
}

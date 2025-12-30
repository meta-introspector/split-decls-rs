// Generated macro for impl_80 (impl)
macro_rules! Depcrateimpl_80 {
() => {
// Module: crate
// Provides: {"impl_80"}
// Dependencies: {}
impl < 'a , const MAX_SIZE : usize > From < & 'a ObjectIdentifier < MAX_SIZE > > for & 'a ObjectIdentifierRef { fn from (oid : & 'a ObjectIdentifier < MAX_SIZE >) -> & 'a ObjectIdentifierRef { oid . as_oid_ref () } }
};
}

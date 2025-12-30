// Generated macro for impl_128 (impl)
macro_rules! Depcrate_schema_ownedimpl_128 {
() => {
// Module: crate::schema::owned
// Provides: {"impl_128"}
// Dependencies: {}
impl From < & NamedType > for OwnedNamedType { fn from (value : & NamedType) -> Self { Self { name : value . name . to_string () , ty : value . ty . into () , } } }
};
}

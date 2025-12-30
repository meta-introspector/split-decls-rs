// Generated macro for impl_137 (impl)
macro_rules! Depcrate_schema_ownedimpl_137 {
() => {
// Module: crate::schema::owned
// Provides: {"impl_137"}
// Dependencies: {}
impl From < & NamedVariant > for OwnedNamedVariant { fn from (value : & NamedVariant) -> Self { Self { name : value . name . to_string () , ty : value . ty . into () , } } }
};
}

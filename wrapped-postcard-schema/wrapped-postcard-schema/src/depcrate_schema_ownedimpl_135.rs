// Generated macro for impl_135 (impl)
macro_rules! Depcrate_schema_ownedimpl_135 {
() => {
// Module: crate::schema::owned
// Provides: {"impl_135"}
// Dependencies: {}
impl From < & NamedValue > for OwnedNamedValue { fn from (value : & NamedValue) -> Self { Self { name : value . name . to_string () , ty : value . ty . into () , } } }
};
}

// Generated macro for impl_127 (impl)
macro_rules! Depcrate_schema_ownedimpl_127 {
() => {
// Module: crate::schema::owned
// Provides: {"impl_127"}
// Dependencies: {}
impl From < & Variant > for OwnedVariant { fn from (value : & Variant) -> Self { Self { name : value . name . into () , data : (& value . data) . into () , } } }
};
}

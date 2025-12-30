// Generated macro for impl_123 (impl)
macro_rules! Depcrate_schema_ownedimpl_123 {
() => {
// Module: crate::schema::owned
// Provides: {"impl_123"}
// Dependencies: {}
impl From < & Data > for OwnedData { fn from (data : & Data) -> Self { match data { Data :: Unit => Self :: Unit , Data :: Newtype (d) => Self :: Newtype (Box :: new ((* d) . into ())) , Data :: Tuple (d) => Self :: Tuple (d . iter () . map (| i | (* i) . into ()) . collect ()) , Data :: Struct (d) => Self :: Struct (d . iter () . map (| i | (* i) . into ()) . collect ()) , } } }
};
}

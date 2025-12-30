// Generated macro for impl_60 (impl)
macro_rules! Depcrate_object_urlimpl_60 {
() => {
// Module: crate::object_url
// Provides: {"impl_60"}
// Dependencies: {}
impl From < web_sys :: Blob > for ObjectUrl { fn from (blob : web_sys :: Blob) -> Self { let url = Url :: create_object_url_with_blob (& blob) . unwrap_throw () ; let inner = Rc :: new (ObjectUrlAllocation { url }) ; Self { inner } } }
};
}

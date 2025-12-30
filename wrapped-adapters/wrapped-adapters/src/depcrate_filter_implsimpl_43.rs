// Generated macro for impl_43 (impl)
macro_rules! Depcrate_filter_implsimpl_43 {
() => {
// Module: crate::filter::impls
// Provides: {"impl_43"}
// Dependencies: {}
impl < D > FilterDataProvider < D , fn (DataIdentifierBorrowed) -> bool > { # [doc = " Creates a [`FilterDataProvider`] that does not do any filtering."] # [doc = ""] # [doc = " Filters can be added using [`Self::with_filter`]."] pub fn new (provider : D , filter_name : & 'static str) -> Self { Self { inner : provider , predicate : | _ | true , filter_name , } } }
};
}

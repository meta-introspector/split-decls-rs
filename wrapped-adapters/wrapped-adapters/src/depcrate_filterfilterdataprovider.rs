// Generated macro for FilterDataProvider (struct)
macro_rules! Depcrate_filterFilterDataProvider {
() => {
// Module: crate::filter
// Provides: {"FilterDataProvider"}
// Dependencies: {}
# [doc = " A data provider that selectively filters out data requests."] # [doc = ""] # [doc = " Data requests that are rejected by the filter will return a [`DataError`] with kind"] # [doc = " [`Filtered`](DataErrorKind::IdentifierNotFound), and they will not be returned"] # [doc = " by [`IterableDynamicDataProvider::iter_ids_for_marker`]."] # [doc = ""] # [doc = " Although this struct can be created directly, the traits in this module provide helper"] # [doc = " functions for common filtering patterns."] # [allow (clippy :: exhaustive_structs)] # [derive (Debug)] pub struct FilterDataProvider < D , F > where F : Fn (DataIdentifierBorrowed) -> bool , { # [doc = " The data provider to which we delegate requests."] pub inner : D , # [doc = " The predicate function. A return value of `true` indicates that the request should"] # [doc = " proceed as normal; a return value of `false` will reject the request."] pub predicate : F , # [doc = " A name for this filter, used in error messages."] pub filter_name : & 'static str , }
};
}

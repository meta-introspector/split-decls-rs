// Generated macro for prelude (module)
macro_rules! Depcrateprelude {
() => {
// Module: crate
// Provides: {"prelude"}
// Dependencies: {}
# [doc = " Core selection of APIs and structures for the ICU4X data provider."] pub mod prelude { # [doc (no_inline)] # [cfg (feature = "serde")] pub use crate :: buf :: AsDeserializingBufferProvider ; # [doc (no_inline)] pub use crate :: buf :: { BufferMarker , BufferProvider } ; # [doc (no_inline)] pub use crate :: { data_marker , data_struct , marker :: DataMarkerExt , request :: AttributeParseError , request :: DataIdentifierBorrowed , BoundDataProvider , DataError , DataErrorKind , DataLocale , DataMarker , DataMarkerAttributes , DataMarkerInfo , DataPayload , DataProvider , DataRequest , DataRequestMetadata , DataResponse , DataResponseMetadata , DryDataProvider , DynamicDataMarker , DynamicDataProvider , DynamicDryDataProvider , ResultDataError , } ; # [cfg (feature = "alloc")] # [doc (no_inline)] pub use crate :: { request :: DataIdentifierCow , IterableDataProvider , IterableDynamicDataProvider , } ; # [doc (no_inline)] pub use icu_locale_core ; # [doc (no_inline)] pub use yoke ; # [doc (no_inline)] pub use zerofrom ; }
};
}

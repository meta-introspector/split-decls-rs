// Generated macro for impl_182 (impl)
macro_rules! Depcrate_requestimpl_182 {
() => {
// Module: crate::request
// Provides: {"impl_182"}
// Dependencies: {}
impl < 'a > DataIdentifierBorrowed < 'a > { # [doc = " Creates a [`DataIdentifierBorrowed`] for a borrowed [`DataLocale`]."] pub fn for_locale (locale : & 'a DataLocale) -> Self { Self { locale , .. Default :: default () } } # [doc = " Creates a [`DataIdentifierBorrowed`] for a borrowed [`DataMarkerAttributes`]."] pub fn for_marker_attributes (marker_attributes : & 'a DataMarkerAttributes) -> Self { Self { marker_attributes , .. Default :: default () } } # [doc = " Creates a [`DataIdentifierBorrowed`] for a borrowed [`DataMarkerAttributes`] and [`DataLocale`]."] pub fn for_marker_attributes_and_locale (marker_attributes : & 'a DataMarkerAttributes , locale : & 'a DataLocale ,) -> Self { Self { marker_attributes , locale , } } # [doc = " Converts this [`DataIdentifierBorrowed`] into a [`DataIdentifierCow<'static>`]."] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [cfg (feature = "alloc")] pub fn into_owned (self) -> DataIdentifierCow < 'static > { DataIdentifierCow { marker_attributes : Cow :: Owned (self . marker_attributes . to_owned ()) , locale : * self . locale , } } # [doc = " Borrows this [`DataIdentifierBorrowed`] as a [`DataIdentifierCow<'a>`]."] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [cfg (feature = "alloc")] pub fn as_cow (self) -> DataIdentifierCow < 'a > { DataIdentifierCow { marker_attributes : Cow :: Borrowed (self . marker_attributes) , locale : * self . locale , } } }
};
}

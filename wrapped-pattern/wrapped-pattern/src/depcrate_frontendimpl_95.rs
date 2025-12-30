// Generated macro for impl_95 (impl)
macro_rules! Depcrate_frontendimpl_95 {
() => {
// Module: crate::frontend
// Provides: {"impl_95"}
// Dependencies: {}
impl < B > Pattern < B > where B : PatternBackend , { # [doc = " Returns an iterator over the [`PatternItem`]s in this pattern."] pub fn iter (& self) -> impl Iterator < Item = PatternItem < '_ , B :: PlaceholderKey < '_ > > > + '_ { B :: iter_items (& self . store) } # [doc = " Returns a [`TryWriteable`] that interpolates items from the given replacement provider"] # [doc = " into this pattern string."] pub fn try_interpolate < 'a , P > (& 'a self , value_provider : P ,) -> impl TryWriteable < Error = B :: Error < 'a > > + fmt :: Display + 'a where P : PlaceholderValueProvider < B :: PlaceholderKey < 'a > , Error = B :: Error < 'a > > + 'a , { WriteablePattern :: < B , P > { store : & self . store , value_provider , } } # [cfg (feature = "alloc")] # [doc = " Interpolates the pattern directly to a string, returning the string or an error."] # [doc = ""] # [doc = " In addition to the error, the lossy fallback string is returned in the failure case."] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] pub fn try_interpolate_to_string < 'a , P > (& 'a self , value_provider : P ,) -> Result < String , (B :: Error < 'a > , String) > where P : PlaceholderValueProvider < B :: PlaceholderKey < 'a > , Error = B :: Error < 'a > > + 'a , { self . try_interpolate (value_provider) . try_write_to_string () . map (| s | s . into_owned ()) . map_err (| (e , s) | (e , s . into_owned ())) } }
};
}

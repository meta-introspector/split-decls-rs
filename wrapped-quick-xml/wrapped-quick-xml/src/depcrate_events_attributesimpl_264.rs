// Generated macro for impl_264 (impl)
macro_rules! Depcrate_events_attributesimpl_264 {
() => {
// Module: crate::events::attributes
// Provides: {"impl_264"}
// Dependencies: {}
impl < 'a > Attr < & 'a [u8] > { # [doc = " Returns the key value"] # [inline] pub const fn key (& self) -> QName < 'a > { QName (match self { Attr :: DoubleQ (key , _) => key , Attr :: SingleQ (key , _) => key , Attr :: Empty (key) => key , Attr :: Unquoted (key , _) => key , }) } # [doc = " Returns the attribute value. For [`Self::Empty`] variant an empty slice"] # [doc = " is returned according to the [HTML specification]."] # [doc = ""] # [doc = " [HTML specification]: https://www.w3.org/TR/2012/WD-html-markup-20120329/syntax.html#syntax-attr-empty"] # [inline] pub const fn value (& self) -> & 'a [u8] { match self { Attr :: DoubleQ (_ , value) => value , Attr :: SingleQ (_ , value) => value , Attr :: Empty (_) => & [] , Attr :: Unquoted (_ , value) => value , } } }
};
}

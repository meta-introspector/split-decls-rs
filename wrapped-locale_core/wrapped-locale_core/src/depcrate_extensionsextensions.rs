// Generated macro for Extensions (struct)
macro_rules! Depcrate_extensionsExtensions {
() => {
// Module: crate::extensions
// Provides: {"Extensions"}
// Dependencies: {}
# [doc = " A map of extensions associated with a given [`Locale`](crate::Locale)."] # [derive (Debug , Default , PartialEq , Eq , Clone , Hash)] # [non_exhaustive] pub struct Extensions { # [doc = " A representation of the data for a Unicode extension, when present in the locale identifier."] pub unicode : Unicode , # [doc = " A representation of the data for a transform extension, when present in the locale identifier."] pub transform : Transform , # [doc = " A representation of the data for a private-use extension, when present in the locale identifier."] pub private : Private , # [doc = " A sequence of any other extensions that are present in the locale identifier but are not formally"] # [doc = " [defined](https://unicode.org/reports/tr35/) and represented explicitly as [`Unicode`], [`Transform`],"] # [doc = " and [`Private`] are."] # [cfg (feature = "alloc")] pub other : Vec < Other > , # [doc = " A sequence of any other extensions that are present in the locale identifier but are not formally"] # [doc = " [defined](https://unicode.org/reports/tr35/) and represented explicitly as [`Unicode`], [`Transform`],"] # [doc = " and [`Private`] are."] # [cfg (not (feature = "alloc"))] pub other : & 'static [Other] , }
};
}

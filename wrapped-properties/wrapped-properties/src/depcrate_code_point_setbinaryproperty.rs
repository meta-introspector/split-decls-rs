// Generated macro for BinaryProperty (trait)
macro_rules! Depcrate_code_point_setBinaryProperty {
() => {
// Module: crate::code_point_set
// Provides: {"BinaryProperty"}
// Dependencies: {}
# [doc = " A binary Unicode character property."] # [doc = ""] # [doc = " The descriptions of most properties are taken from [`TR44`], the documentation for the"] # [doc = " Unicode Character Database.  Some properties are instead defined in [`TR18`], the"] # [doc = " documentation for Unicode regular expressions. In particular, Annex C of this document"] # [doc = " defines properties for POSIX compatibility."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚫 This trait is sealed; it cannot be implemented by user code. If an API requests an item that implements this"] # [doc = " trait, please consider using a type from the implementors listed below."] # [doc = " </div>"] # [doc = ""] # [doc = " [`TR44`]: https://www.unicode.org/reports/tr44"] # [doc = " [`TR18`]: https://www.unicode.org/reports/tr18"] pub trait BinaryProperty : crate :: private :: Sealed + Sized { # [doc (hidden)] type DataMarker : DataMarker < DataStruct = PropertyCodePointSet < 'static > > ; # [doc (hidden)] # [cfg (feature = "compiled_data")] const SINGLETON : & 'static PropertyCodePointSet < 'static > ; # [doc = " The name of this property"] const NAME : & 'static [u8] ; # [doc = " The abbreviated name of this property, if it exists, otherwise the name"] const SHORT_NAME : & 'static [u8] ; # [doc = " Convenience method for `CodePointSetData::new().contains(ch)`"] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [cfg (feature = "compiled_data")] fn for_char (ch : char) -> bool { CodePointSetData :: new :: < Self > () . contains (ch) } }
};
}

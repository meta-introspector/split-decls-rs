// Generated macro for EnumeratedProperty (trait)
macro_rules! Depcrate_code_point_mapEnumeratedProperty {
() => {
// Module: crate::code_point_map
// Provides: {"EnumeratedProperty"}
// Dependencies: {}
# [doc = " A Unicode character property that assigns a value to each code point."] # [doc = ""] # [doc = " The descriptions of most properties are taken from [`TR44`], the documentation for the"] # [doc = " Unicode Character Database."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚫 This trait is sealed; it cannot be implemented by user code. If an API requests an item that implements this"] # [doc = " trait, please consider using a type from the implementors listed below."] # [doc = " </div>"] # [doc = ""] # [doc = " [`TR44`]: https://www.unicode.org/reports/tr44"] pub trait EnumeratedProperty : crate :: private :: Sealed + TrieValue { # [doc (hidden)] type DataMarker : DataMarker < DataStruct = PropertyCodePointMap < 'static , Self > > ; # [doc (hidden)] # [cfg (feature = "compiled_data")] const SINGLETON : & 'static PropertyCodePointMap < 'static , Self > ; # [doc = " The name of this property"] const NAME : & 'static [u8] ; # [doc = " The abbreviated name of this property, if it exists, otherwise the name"] const SHORT_NAME : & 'static [u8] ; # [doc = " Convenience method for `CodePointMapData::new().get(ch)`"] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [cfg (feature = "compiled_data")] fn for_char (ch : char) -> Self { CodePointMapData :: new () . get (ch) } }
};
}

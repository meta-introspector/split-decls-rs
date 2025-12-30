// Generated macro for impl_24 (impl)
macro_rules! Depcrate_colorimpl_24 {
() => {
// Module: crate::color
// Provides: {"impl_24"}
// Dependencies: {}
impl std :: str :: FromStr for OutType { type Err = ColorError ; fn from_str (s : & str) -> Result < OutType , ColorError > { match & * s . to_lowercase () { "path" => Ok (OutType :: Path) , "line" => Ok (OutType :: Line) , "column" => Ok (OutType :: Column) , "match" => Ok (OutType :: Match) , "highlight" => Ok (OutType :: Highlight) , _ => Err (ColorError :: UnrecognizedOutType (s . to_string ())) , } } }
};
}

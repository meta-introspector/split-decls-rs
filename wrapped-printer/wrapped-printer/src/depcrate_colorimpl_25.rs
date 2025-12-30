// Generated macro for impl_25 (impl)
macro_rules! Depcrate_colorimpl_25 {
() => {
// Module: crate::color
// Provides: {"impl_25"}
// Dependencies: {}
impl std :: str :: FromStr for SpecType { type Err = ColorError ; fn from_str (s : & str) -> Result < SpecType , ColorError > { match & * s . to_lowercase () { "fg" => Ok (SpecType :: Fg) , "bg" => Ok (SpecType :: Bg) , "style" => Ok (SpecType :: Style) , "none" => Ok (SpecType :: None) , _ => Err (ColorError :: UnrecognizedSpecType (s . to_string ())) , } } }
};
}

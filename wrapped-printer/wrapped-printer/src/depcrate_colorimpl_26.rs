// Generated macro for impl_26 (impl)
macro_rules! Depcrate_colorimpl_26 {
() => {
// Module: crate::color
// Provides: {"impl_26"}
// Dependencies: {}
impl std :: str :: FromStr for Style { type Err = ColorError ; fn from_str (s : & str) -> Result < Style , ColorError > { match & * s . to_lowercase () { "bold" => Ok (Style :: Bold) , "nobold" => Ok (Style :: NoBold) , "intense" => Ok (Style :: Intense) , "nointense" => Ok (Style :: NoIntense) , "underline" => Ok (Style :: Underline) , "nounderline" => Ok (Style :: NoUnderline) , "italic" => Ok (Style :: Italic) , "noitalic" => Ok (Style :: NoItalic) , _ => Err (ColorError :: UnrecognizedStyle (s . to_string ())) , } } }
};
}

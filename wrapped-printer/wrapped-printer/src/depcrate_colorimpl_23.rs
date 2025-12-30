// Generated macro for impl_23 (impl)
macro_rules! Depcrate_colorimpl_23 {
() => {
// Module: crate::color
// Provides: {"impl_23"}
// Dependencies: {}
impl std :: str :: FromStr for UserColorSpec { type Err = ColorError ; fn from_str (s : & str) -> Result < UserColorSpec , ColorError > { let pieces : Vec < & str > = s . split (':') . collect () ; if pieces . len () <= 1 || pieces . len () > 3 { return Err (ColorError :: InvalidFormat (s . to_string ())) ; } let otype : OutType = pieces [0] . parse () ? ; match pieces [1] . parse () ? { SpecType :: None => { Ok (UserColorSpec { ty : otype , value : SpecValue :: None }) } SpecType :: Style => { if pieces . len () < 3 { return Err (ColorError :: InvalidFormat (s . to_string ())) ; } let style : Style = pieces [2] . parse () ? ; Ok (UserColorSpec { ty : otype , value : SpecValue :: Style (style) }) } SpecType :: Fg => { if pieces . len () < 3 { return Err (ColorError :: InvalidFormat (s . to_string ())) ; } let color : Color = pieces [2] . parse () . map_err (ColorError :: from_parse_error) ? ; Ok (UserColorSpec { ty : otype , value : SpecValue :: Fg (color) }) } SpecType :: Bg => { if pieces . len () < 3 { return Err (ColorError :: InvalidFormat (s . to_string ())) ; } let color : Color = pieces [2] . parse () . map_err (ColorError :: from_parse_error) ? ; Ok (UserColorSpec { ty : otype , value : SpecValue :: Bg (color) }) } } } }
};
}

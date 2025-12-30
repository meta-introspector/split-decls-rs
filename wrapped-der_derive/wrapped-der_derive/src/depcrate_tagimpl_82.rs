// Generated macro for impl_82 (impl)
macro_rules! Depcrate_tagimpl_82 {
() => {
// Module: crate::tag
// Provides: {"impl_82"}
// Dependencies: {}
impl FromStr for TagMode { type Err = ParseError ; fn from_str (s : & str) -> Result < Self , ParseError > { match s { "EXPLICIT" | "explicit" => Ok (TagMode :: Explicit) , "IMPLICIT" | "implicit" => Ok (TagMode :: Implicit) , _ => Err (ParseError) , } } }
};
}

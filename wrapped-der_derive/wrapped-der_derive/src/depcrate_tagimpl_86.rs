// Generated macro for impl_86 (impl)
macro_rules! Depcrate_tagimpl_86 {
() => {
// Module: crate::tag
// Provides: {"impl_86"}
// Dependencies: {}
impl FromStr for TagNumber { type Err = ParseError ; fn from_str (s : & str) -> Result < Self , ParseError > { s . parse :: < u32 > () . map (Self) . map_err (| _ | ParseError) } }
};
}

// Generated macro for Float (trait)
macro_rules! Depcrate_parseFloat {
() => {
// Module: crate::parse
// Provides: {"Float"}
// Dependencies: {}
pub trait Float : Sized { fn parse (float : & str) -> Result < Self > ; fn try_from_parsed_float (parsed : ParsedFloat , ron : & str) -> Result < Self > ; }
};
}

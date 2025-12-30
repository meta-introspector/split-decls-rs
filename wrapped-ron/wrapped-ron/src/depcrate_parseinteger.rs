// Generated macro for Integer (trait)
macro_rules! Depcrate_parseInteger {
() => {
// Module: crate::parse
// Provides: {"Integer"}
// Dependencies: {}
pub trait Integer : Sized { fn parse (parser : & mut Parser , sign : i8) -> Result < Self > ; fn try_from_parsed_integer (parsed : ParsedInteger , ron : & str) -> Result < Self > ; }
};
}

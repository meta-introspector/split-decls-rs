// Generated macro for sign (function)
macro_rules! Depcrate_character_completesign {
() => {
// Module: crate::character::complete
// Provides: {"sign"}
// Dependencies: {}
pub (crate) fn sign < T , E : ParseError < T > > (input : T) -> IResult < T , bool , E > where T : Clone + Input , T : for < 'a > Compare < & 'a [u8] > , { use crate :: bytes :: complete :: tag ; use crate :: combinator :: value ; let (i , opt_sign) = opt (alt ((value (false , tag (& b"-" [..])) , value (true , tag (& b"+" [..])) ,))) . parse (input) ? ; let sign = opt_sign . unwrap_or (true) ; Ok ((i , sign)) }
};
}

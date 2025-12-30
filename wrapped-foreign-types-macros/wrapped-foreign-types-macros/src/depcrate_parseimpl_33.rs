// Generated macro for impl_33 (impl)
macro_rules! Depcrate_parseimpl_33 {
() => {
// Module: crate::parse
// Provides: {"impl_33"}
// Dependencies: {}
impl Parse for ForeignType { fn parse (input : ParseStream) -> parse :: Result < ForeignType > { let attrs = input . call (Attribute :: parse_outer) ? ; let visibility = input . parse () ? ; input . parse :: < Token ! [unsafe] > () ? ; input . parse :: < Token ! [type] > () ? ; let name = input . parse () ? ; let generics = input . parse () ? ; let oibits = input . call (parse_oibits) ? ; let inner ; braced ! (inner in input) ; let ctype = inner . call (parse_type :: < kw :: CType >) ? ; let phantom_data = inner . call (parse_phantom_data) ? ; let drop = inner . call (parse_fn :: < kw :: drop >) ? ; let clone = inner . call (parse_clone) ? ; Ok (ForeignType { attrs , visibility , name , generics , oibits , ctype , phantom_data , drop , clone , }) } }
};
}

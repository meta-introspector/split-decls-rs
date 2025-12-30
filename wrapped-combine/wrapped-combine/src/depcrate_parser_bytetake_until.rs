// Generated macro for take_until (macro)
macro_rules! Depcrate_parser_bytetake_until {
() => {
// Module: crate::parser::byte
// Provides: {"take_until"}
// Dependencies: {}
macro_rules ! take_until { ($ (# [$ attr : meta]) * $ type_name : ident , $ func_name : ident , $ memchr : ident , $ ($ param : ident) ,+) => { parser ! { # [derive (Clone)] pub struct $ type_name ; type PartialState = usize ; $ (# [$ attr]) * pub fn $ func_name [Input] ($ ($ param : u8) ,*) (Input) -> Input :: Range where [Input : RangeStream , Input :: Range : AsRef < [u8] > + crate :: stream :: Range ,] { take_fn (move | haystack : Input :: Range | { let haystack = haystack . as_ref () ; match :: memchr ::$ memchr ($ (*$ param) ,+ , haystack) { Some (i) => TakeRange :: Found (i) , None => TakeRange :: NotFound (haystack . len ()) , } }) } } } }
};
}

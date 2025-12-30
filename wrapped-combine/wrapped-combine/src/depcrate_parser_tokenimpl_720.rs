// Generated macro for impl_720 (impl)
macro_rules! Depcrate_parser_tokenimpl_720 {
() => {
// Module: crate::parser::token
// Provides: {"impl_720"}
// Dependencies: {}
impl < Input , P > Parser < Input > for Satisfy < Input , P > where Input : Stream , P : FnMut (Input :: Token) -> bool , { type Output = Input :: Token ; type PartialState = () ; # [inline] fn parse_lazy (& mut self , input : & mut Input) -> ParseResult < Self :: Output , Input :: Error > { satisfy_impl (input , | c | { if (self . predicate) (c . clone ()) { Some (c) } else { None } }) } }
};
}

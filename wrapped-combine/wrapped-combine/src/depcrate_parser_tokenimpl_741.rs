// Generated macro for impl_741 (impl)
macro_rules! Depcrate_parser_tokenimpl_741 {
() => {
// Module: crate::parser::token
// Provides: {"impl_741"}
// Dependencies: {}
impl < Input , T > Parser < Input > for NoneOf < T , Input > where T : Clone + IntoIterator < Item = Input :: Token > , Input : Stream , Input :: Token : PartialEq , { type Output = Input :: Token ; type PartialState = () ; # [inline] fn parse_lazy (& mut self , input : & mut Input) -> ParseResult < Input :: Token , Input :: Error > { satisfy (| c | self . tokens . clone () . into_iter () . all (| t | t != c)) . parse_lazy (input) } }
};
}

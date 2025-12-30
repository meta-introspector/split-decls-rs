// Generated macro for impl_738 (impl)
macro_rules! Depcrate_parser_tokenimpl_738 {
() => {
// Module: crate::parser::token
// Provides: {"impl_738"}
// Dependencies: {}
impl < Input , T > Parser < Input > for OneOf < T , Input > where T : Clone + IntoIterator < Item = Input :: Token > , Input : Stream , Input :: Token : PartialEq , { type Output = Input :: Token ; type PartialState = () ; # [inline] fn parse_lazy (& mut self , input : & mut Input) -> ParseResult < Input :: Token , Input :: Error > { satisfy (| c | self . tokens . clone () . into_iter () . any (| t | t == c)) . parse_lazy (input) } fn add_error (& mut self , errors : & mut Tracked < < Input as StreamOnce > :: Error >) { for expected in self . tokens . clone () { errors . error . add_expected (error :: Token (expected)) ; } } }
};
}

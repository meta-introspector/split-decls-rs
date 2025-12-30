// Generated macro for impl_744 (impl)
macro_rules! Depcrate_parser_tokenimpl_744 {
() => {
// Module: crate::parser::token
// Provides: {"impl_744"}
// Dependencies: {}
impl < Input , T > Parser < Input > for Value < Input , T > where Input : Stream , T : Clone , { type Output = T ; type PartialState = () ; # [inline] fn parse_lazy (& mut self , _ : & mut Input) -> ParseResult < T , Input :: Error > { PeekOk (self . 0 . clone ()) } }
};
}

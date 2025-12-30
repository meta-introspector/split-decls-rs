// Generated macro for impl_747 (impl)
macro_rules! Depcrate_parser_tokenimpl_747 {
() => {
// Module: crate::parser::token
// Provides: {"impl_747"}
// Dependencies: {}
impl < Input , F , R > Parser < Input > for Produce < Input , F > where Input : Stream , F : FnMut () -> R , { type Output = R ; type PartialState = () ; # [inline] fn parse_lazy (& mut self , _ : & mut Input) -> ParseResult < R , Input :: Error > { PeekOk ((self . 0) ()) } }
};
}

// Generated macro for impl_540 (impl)
macro_rules! Depcrate_parser_functionimpl_540 {
() => {
// Module: crate::parser::function
// Provides: {"impl_540"}
// Dependencies: {}
impl < Input , E , O > Parser < Input > for EnvParser < E , Input , O > where E : Clone , Input : Stream , { type Output = O ; type PartialState = () ; # [inline] fn parse_lazy (& mut self , input : & mut Input) -> ParseResult < O , Input :: Error > { (self . parser) (self . env . clone () , input) . into () } }
};
}

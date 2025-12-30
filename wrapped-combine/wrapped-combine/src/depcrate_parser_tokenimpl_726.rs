// Generated macro for impl_726 (impl)
macro_rules! Depcrate_parser_tokenimpl_726 {
() => {
// Module: crate::parser::token
// Provides: {"impl_726"}
// Dependencies: {}
impl < Input > Parser < Input > for Token < Input > where Input : Stream , Input :: Token : PartialEq + Clone , { type Output = Input :: Token ; type PartialState = () ; # [inline] fn parse_lazy (& mut self , input : & mut Input) -> ParseResult < Input :: Token , Input :: Error > { satisfy_impl (input , | c | if c == self . c { Some (c) } else { None }) } fn add_error (& mut self , errors : & mut Tracked < < Input as StreamOnce > :: Error >) { errors . error . add_expected (error :: Token (self . c . clone ())) ; } }
};
}

// Generated macro for impl_760 (impl)
macro_rules! Depcrate_parserimpl_760 {
() => {
// Module: crate::parser
// Provides: {"impl_760"}
// Dependencies: {}
impl ParseMode for FirstMode { # [inline] fn is_first (self) -> bool { true } # [inline] fn set_first (& mut self) { } fn parse < P , Input > (self , parser : & mut P , input : & mut Input , state : & mut P :: PartialState ,) -> ParseResult < P :: Output , Input :: Error > where P : Parser < Input > , Input : Stream , { parser . parse_mode_impl (FirstMode , input , state) } }
};
}

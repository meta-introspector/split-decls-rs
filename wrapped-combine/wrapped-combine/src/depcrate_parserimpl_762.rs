// Generated macro for impl_762 (impl)
macro_rules! Depcrate_parserimpl_762 {
() => {
// Module: crate::parser
// Provides: {"impl_762"}
// Dependencies: {}
impl ParseMode for PartialMode { # [inline] fn is_first (self) -> bool { self . first } # [inline] fn set_first (& mut self) { self . first = true ; } fn parse < P , Input > (self , parser : & mut P , input : & mut Input , state : & mut P :: PartialState ,) -> ParseResult < P :: Output , Input :: Error > where P : Parser < Input > , Input : Stream , { if self . is_first () { parser . parse_mode_impl (FirstMode , input , state) } else { parser . parse_mode_impl (self , input , state) } } }
};
}

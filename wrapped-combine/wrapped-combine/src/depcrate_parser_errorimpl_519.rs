// Generated macro for impl_519 (impl)
macro_rules! Depcrate_parser_errorimpl_519 {
() => {
// Module: crate::parser::error
// Provides: {"impl_519"}
// Dependencies: {}
impl < Input , T , E > Parser < Input > for Unexpected < Input , T , E > where Input : Stream , E : for < 's > ErrorInfo < 's , Input :: Token , Input :: Range > , { type Output = T ; type PartialState = () ; # [inline] fn parse_lazy (& mut self , input : & mut Input) -> ParseResult < T , < Input as StreamOnce > :: Error > { PeekErr (< Input as StreamOnce > :: Error :: empty (input . position ()) . into ()) } fn add_error (& mut self , errors : & mut Tracked < < Input as StreamOnce > :: Error >) { errors . error . add (StreamError :: unexpected (& self . 0)) ; } }
};
}

// Generated macro for impl_557 (impl)
macro_rules! Depcrate_parser_rangeimpl_557 {
() => {
// Module: crate::parser::range
// Provides: {"impl_557"}
// Dependencies: {}
impl < Input > Parser < Input > for Take < Input > where Input : RangeStream , { type Output = Input :: Range ; type PartialState = () ; # [inline] fn parse_lazy (& mut self , input : & mut Input ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > { uncons_range (input , self . 0) } }
};
}

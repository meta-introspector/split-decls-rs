// Generated macro for impl_549 (impl)
macro_rules! Depcrate_parser_rangeimpl_549 {
() => {
// Module: crate::parser::range
// Provides: {"impl_549"}
// Dependencies: {}
impl < Input > Parser < Input > for Range < Input > where Input : RangeStream , Input :: Range : PartialEq + crate :: stream :: Range , { type Output = Input :: Range ; type PartialState = () ; # [inline] fn parse_lazy (& mut self , input : & mut Input ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > { use crate :: stream :: Range ; let position = input . position () ; match input . uncons_range (self . 0 . len ()) { Ok (other) => { if other == self . 0 { CommitOk (other) } else { PeekErr (Input :: Error :: empty (position) . into ()) } } Err (err) => wrap_stream_error (input , err) , } } fn add_error (& mut self , errors : & mut Tracked < < Input as StreamOnce > :: Error >) { errors . error . add_expected (error :: Range (self . 0 . clone ())) ; } }
};
}

// Generated macro for impl_571 (impl)
macro_rules! Depcrate_parser_rangeimpl_571 {
() => {
// Module: crate::parser::range
// Provides: {"impl_571"}
// Dependencies: {}
impl < Input , F , R > Parser < Input > for TakeFn < F , Input > where F : FnMut (Input :: Range) -> R , R : Into < TakeRange > , Input : RangeStream , Input :: Range : crate :: stream :: Range , { type Output = Input :: Range ; type PartialState = usize ; parse_mode ! (Input) ; # [inline] fn parse_mode < M > (& mut self , mode : M , input : & mut Input , offset : & mut Self :: PartialState ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > where M : ParseMode , { let checkpoint = input . checkpoint () ; if mode . is_first () { * offset = 0 ; } else { let _ = input . uncons_range (* offset) ; } match (self . searcher) (input . range ()) . into () { TakeRange :: Found (i) => { ctry ! (input . reset (checkpoint) . committed ()) ; let result = uncons_range (input , * offset + i) ; if result . is_ok () { * offset = 0 ; } result } TakeRange :: NotFound (next_offset) => { * offset = next_offset ; let range = input . range () ; let _ = input . uncons_range (range . len ()) ; let position = input . position () ; ctry ! (input . reset (checkpoint) . committed ()) ; let err = Input :: Error :: from_error (position , StreamError :: end_of_input ()) ; if ! input . is_partial () && range . is_empty () { PeekErr (err . into ()) } else { CommitErr (err) } } } } }
};
}

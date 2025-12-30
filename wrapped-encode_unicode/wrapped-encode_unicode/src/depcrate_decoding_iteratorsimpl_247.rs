// Generated macro for impl_247 (impl)
macro_rules! Depcrate_decoding_iteratorsimpl_247 {
() => {
// Module: crate::decoding_iterators
// Provides: {"impl_247"}
// Dependencies: {}
impl < 'a > DoubleEndedIterator for Utf8CharDecoder < 'a > { fn next_back (& mut self) -> Option < Self :: Item > { if self . index < self . slice . len () { let extras = self . slice . iter () . rev () . take_while (| & b | b & 0b1100_0000 == 0b1000_0000) . count () ; let starts = self . slice . len () - (extras + 1) ; match Utf8Char :: from_slice_start (& self . slice [starts ..]) { Ok ((u8c , len)) if len == 1 + extras => { self . slice = & self . slice [.. starts] ; Some ((starts , Ok (u8c) , len)) } , Err (e) if extras == 0 => { self . slice = & self . slice [.. self . slice . len () - 1] ; Some ((self . slice . len () - 1 , Err (e) , 1)) } , _ => { self . slice = & self . slice [.. self . slice . len () - 1] ; Some ((self . slice . len () - 1 , Err (Utf8Error { kind : UnexpectedContinuationByte }) , 1)) } , } } else { None } } }
};
}

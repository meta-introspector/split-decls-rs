// Generated macro for impl_246 (impl)
macro_rules! Depcrate_decoding_iteratorsimpl_246 {
() => {
// Module: crate::decoding_iterators
// Provides: {"impl_246"}
// Dependencies: {}
impl < 'a > Iterator for Utf8CharDecoder < 'a > { type Item = (usize , Result < Utf8Char , Utf8Error > , usize) ; fn next (& mut self) -> Option < Self :: Item > { let start = self . index ; match Utf8Char :: from_slice_start (& self . slice [self . index ..]) { Ok ((u8c , len)) => { self . index += len ; Some ((start , Ok (u8c) , len)) } , Err (_) if self . slice . len () <= self . index => None , Err (e) => { self . index += 1 ; Some ((start , Err (e) , 1)) } } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let bytes = self . slice . len () - self . index ; (bytes / 4 , Some (bytes)) } }
};
}

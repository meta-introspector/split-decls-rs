// Generated macro for impl_257 (impl)
macro_rules! Depcrate_decoding_iteratorsimpl_257 {
() => {
// Module: crate::decoding_iterators
// Provides: {"impl_257"}
// Dependencies: {}
impl < 'a > Iterator for Utf16CharDecoder < 'a > { type Item = (usize , Result < Utf16Char , Utf16PairError > , usize) ; # [inline] fn next (& mut self) -> Option < Self :: Item > { let start = self . index ; match Utf16Char :: from_slice_start (self . as_slice ()) { Ok ((u16c , len)) => { self . index += len ; Some ((start , Ok (u16c) , len)) } , Err (EmptySlice) => None , Err (FirstIsTrailingSurrogate) => { self . index += 1 ; Some ((start , Err (UnexpectedTrailingSurrogate) , 1)) } , Err (SecondIsNotTrailingSurrogate) => { self . index += 1 ; Some ((start , Err (UnmatchedLeadingSurrogate) , 1)) } , Err (MissingSecond) => { self . index = self . slice . len () ; Some ((start , Err (Incomplete) , 1)) } } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let units = self . slice . len () - self . index ; (units / 2 , Some (units)) } }
};
}

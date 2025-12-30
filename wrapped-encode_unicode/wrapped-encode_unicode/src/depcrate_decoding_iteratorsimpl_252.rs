// Generated macro for impl_252 (impl)
macro_rules! Depcrate_decoding_iteratorsimpl_252 {
() => {
// Module: crate::decoding_iterators
// Provides: {"impl_252"}
// Dependencies: {}
impl < B : Borrow < u16 > , I : Iterator < Item = B > > Iterator for Utf16CharMerger < B , I > { type Item = Result < Utf16Char , Utf16PairError > ; fn next (& mut self) -> Option < Self :: Item > { let first = self . prev . take () . or_else (| | self . iter . next ()) ; first . map (| first | unsafe { match first . borrow () . utf16_needs_extra_unit () { Ok (false) => Ok (Utf16Char :: from_array_unchecked ([* first . borrow () , 0])) , Ok (true) => match self . iter . next () { Some (second) => match second . borrow () . utf16_needs_extra_unit () { Err (Utf16FirstUnitError) => Ok (Utf16Char :: from_tuple_unchecked ((* first . borrow () , Some (* second . borrow ())))) , Ok (_) => { self . prev = Some (second) ; Err (Utf16PairError :: UnmatchedLeadingSurrogate) } } , None => Err (Utf16PairError :: Incomplete) } , Err (Utf16FirstUnitError) => Err (Utf16PairError :: UnexpectedTrailingSurrogate) , } }) } fn size_hint (& self) -> (usize , Option < usize >) { let (iter_min , iter_max) = self . iter . size_hint () ; let min = iter_min / 2 ; let max = match (iter_max , & self . prev) { (Some (max) , & Some (_)) => max . checked_add (1) , (max , _) => max , } ; (min , max) } }
};
}

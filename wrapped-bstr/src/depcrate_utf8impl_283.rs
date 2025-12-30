// Generated macro for impl_283 (impl)
macro_rules! Depcrate_utf8impl_283 {
() => {
// Module: crate::utf8
// Provides: {"impl_283"}
// Dependencies: {}
impl < 'a > Iterator for Utf8Chunks < 'a > { type Item = Utf8Chunk < 'a > ; # [inline] fn next (& mut self) -> Option < Utf8Chunk < 'a > > { if self . bytes . is_empty () { return None ; } match validate (self . bytes) { Ok (()) => { let valid = self . bytes ; self . bytes = & [] ; Some (Utf8Chunk { valid : unsafe { str :: from_utf8_unchecked (valid) } , invalid : [] . as_bstr () , incomplete : false , }) } Err (e) => { let (valid , rest) = self . bytes . split_at (e . valid_up_to ()) ; let valid = unsafe { str :: from_utf8_unchecked (valid) } ; let (invalid_len , incomplete) = match e . error_len () { Some (n) => (n , false) , None => (rest . len () , true) , } ; let (invalid , rest) = rest . split_at (invalid_len) ; self . bytes = rest ; Some (Utf8Chunk { valid , invalid : invalid . as_bstr () , incomplete , }) } } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { if self . bytes . is_empty () { (0 , Some (0)) } else { (1 , Some (self . bytes . len ())) } } }
};
}

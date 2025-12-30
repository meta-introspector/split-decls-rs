// Generated macro for impl_36 (impl)
macro_rules! Depcrateimpl_36 {
() => {
// Module: crate
// Provides: {"impl_36"}
// Dependencies: {}
impl < T , B : BitBlock > Iterator for BlockIter < T , B > where T : Iterator < Item = B > , { type Item = usize ; fn next (& mut self) -> Option < usize > { while self . head == B :: zero () { match self . tail . next () { Some (w) => self . head = w , None => return None , } self . head_offset += B :: bits () ; } let k = (self . head & (! self . head + B :: one ())) - B :: one () ; self . head = self . head & (self . head - B :: one ()) ; Some (self . head_offset + (B :: count_ones (k))) } fn count (self) -> usize { self . head . count_ones () + self . tail . map (| block | block . count_ones ()) . sum :: < usize > () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { match self . tail . size_hint () { (_ , Some (h)) => (0 , Some ((1 + h) * B :: bits ())) , _ => (0 , None) , } } }
};
}

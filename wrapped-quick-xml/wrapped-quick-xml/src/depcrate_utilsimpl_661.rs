// Generated macro for impl_661 (impl)
macro_rules! Depcrate_utilsimpl_661 {
() => {
// Module: crate::utils
// Provides: {"impl_661"}
// Dependencies: {}
impl < 'a > Iterator for CDataIterator < 'a > { type Item = & 'a str ; fn next (& mut self) -> Option < & 'a str > { if self . finished { return None ; } for gt in memchr :: memchr_iter (b'>' , self . unprocessed . as_bytes ()) { let (slice , rest) = self . unprocessed . split_at (gt) ; if slice . ends_with ("]]") { self . unprocessed = rest ; return Some (slice) ; } } self . finished = true ; Some (self . unprocessed) } }
};
}

// Generated macro for impl_5 (impl)
macro_rules! Depcrate_capturesimpl_5 {
() => {
// Module: crate::captures
// Provides: {"impl_5"}
// Dependencies: {}
impl Captures { # [doc = " New returns an instance of Found."] pub (crate) fn new (buf : Vec < u8 > , matches : Vec < Match >) -> Self { Self { buf , matches } } # [doc = " is_empty verifies if any matches were actually found."] pub fn is_empty (& self) -> bool { self . matches . is_empty () } # [doc = " get returns a match by index."] pub fn get (& self , index : usize) -> Option < & [u8] > { self . matches . get (index) . map (| m | & self . buf [m . start () .. m . end ()]) } # [doc = " Matches returns a list of matches."] pub fn matches (& self) -> MatchIter < '_ > { MatchIter :: new (self) } # [doc = " before returns a bytes before match."] pub fn before (& self) -> & [u8] { & self . buf [.. self . left_most_index ()] } # [doc = " as_bytes returns all bytes involved in a match, e.g. before the match and"] # [doc = " in a match itself."] # [doc = ""] # [doc = " In most cases the returned value equeals to concatanted [Self::before] and [Self::matches]."] # [doc = " But sometimes like in case of [crate::Regex] it may have a grouping so [Self::matches] might overlap, therefore"] # [doc = " it will not longer be true."] pub fn as_bytes (& self) -> & [u8] { & self . buf } fn left_most_index (& self) -> usize { self . matches . iter () . map (| m | m . start ()) . min () . unwrap_or_default () } pub (crate) fn right_most_index (matches : & [Match]) -> usize { matches . iter () . map (| m | m . end ()) . max () . unwrap_or_default () } }
};
}

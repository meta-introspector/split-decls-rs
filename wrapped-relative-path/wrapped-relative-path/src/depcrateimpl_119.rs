// Generated macro for impl_119 (impl)
macro_rules! Depcrateimpl_119 {
() => {
// Module: crate
// Provides: {"impl_119"}
// Dependencies: {}
impl DoubleEndedIterator for Components < '_ > { fn next_back (& mut self) -> Option < Self :: Item > { self . source = self . source . trim_end_matches (SEP) ; let slice = match self . source . rfind (SEP) { Some (i) => { let (rest , slice) = self . source . split_at (i + 1) ; self . source = rest . trim_end_matches (SEP) ; slice } None => mem :: take (& mut self . source) , } ; match slice { "" => None , CURRENT_STR => Some (Component :: CurDir) , PARENT_STR => Some (Component :: ParentDir) , slice => Some (Component :: Normal (slice)) , } } }
};
}

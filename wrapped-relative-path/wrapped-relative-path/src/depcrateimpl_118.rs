// Generated macro for impl_118 (impl)
macro_rules! Depcrateimpl_118 {
() => {
// Module: crate
// Provides: {"impl_118"}
// Dependencies: {}
impl < 'a > Iterator for Components < 'a > { type Item = Component < 'a > ; fn next (& mut self) -> Option < Self :: Item > { self . source = self . source . trim_start_matches (SEP) ; let slice = match self . source . find (SEP) { Some (i) => { let (slice , rest) = self . source . split_at (i) ; self . source = rest . trim_start_matches (SEP) ; slice } None => mem :: take (& mut self . source) , } ; match slice { "" => None , CURRENT_STR => Some (Component :: CurDir) , PARENT_STR => Some (Component :: ParentDir) , slice => Some (Component :: Normal (slice)) , } } }
};
}

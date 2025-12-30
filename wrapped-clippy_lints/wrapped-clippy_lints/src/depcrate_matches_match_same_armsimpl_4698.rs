// Generated macro for impl_4698 (impl)
macro_rules! Depcrate_matches_match_same_armsimpl_4698 {
() => {
// Module: crate::matches::match_same_arms
// Provides: {"impl_4698"}
// Dependencies: {}
impl PatRange { fn contains (& self , x : u128) -> bool { x >= self . start && match self . bounds { RangeEnd :: Included => x <= self . end , RangeEnd :: Excluded => x < self . end , } } fn overlaps (& self , other : & Self) -> bool { (match self . bounds { RangeEnd :: Included => self . end >= other . start , RangeEnd :: Excluded => self . end > other . start , } && match other . bounds { RangeEnd :: Included => self . start <= other . end , RangeEnd :: Excluded => self . start < other . end , }) } }
};
}

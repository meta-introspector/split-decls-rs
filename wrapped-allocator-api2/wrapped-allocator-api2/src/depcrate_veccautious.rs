// Generated macro for cautious (function)
macro_rules! Depcrate_veccautious {
() => {
// Module: crate::vec
// Provides: {"cautious"}
// Dependencies: {}
# [cfg (feature = "serde")] pub fn cautious (hint : Option < usize >) -> usize { cmp :: min (hint . unwrap_or (0) , 4096) }
};
}

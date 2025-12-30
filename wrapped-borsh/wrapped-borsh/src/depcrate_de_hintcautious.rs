// Generated macro for cautious (function)
macro_rules! Depcrate_de_hintcautious {
() => {
// Module: crate::de::hint
// Provides: {"cautious"}
// Dependencies: {}
# [inline] pub fn cautious < T > (hint : u32) -> usize { let el_size = core :: mem :: size_of :: < T > () as u32 ; core :: cmp :: max (core :: cmp :: min (hint , 4096 / el_size) , 1) as usize }
};
}

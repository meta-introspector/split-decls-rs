// Generated macro for Danger (enum)
macro_rules! Depcrate_header_mapDanger {
() => {
// Module: crate::header::map
// Provides: {"Danger"}
// Dependencies: {}
# [doc = " Tracks the header map danger level! This relates to the adaptive hashing"] # [doc = " algorithm. A HeaderMap starts in the \"green\" state, when a large number of"] # [doc = " collisions are detected, it transitions to the yellow state. At this point,"] # [doc = " the header map will either grow and switch back to the green state OR it"] # [doc = " will transition to the red state."] # [doc = ""] # [doc = " When in the red state, a safe hashing algorithm is used and all values in"] # [doc = " the header map have to be rehashed."] # [derive (Clone)] enum Danger { Green , Yellow , Red (RandomState) , }
};
}

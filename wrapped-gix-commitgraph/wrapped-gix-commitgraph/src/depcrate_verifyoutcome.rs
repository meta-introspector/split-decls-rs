// Generated macro for Outcome (struct)
macro_rules! Depcrate_verifyOutcome {
() => {
// Module: crate::verify
// Provides: {"Outcome"}
// Dependencies: {}
# [doc = " Statistics gathered while verifying the integrity of the graph as returned by [`Graph::verify_integrity()`]."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize , serde :: Serialize))] pub struct Outcome { # [doc = " The length of the longest path between any two commits in this graph."] # [doc = ""] # [doc = " For example, this will be `Some(9)` for a commit graph containing 10 linear commits."] # [doc = " This will be `Some(0)` for a commit graph containing 0 or 1 commits."] # [doc = " If the longest path length is too large to fit in a [u32], then this will be [None]."] pub longest_path_length : Option < u32 > , # [doc = " The total number of commits traversed."] pub num_commits : u32 , # [doc = " A mapping of `N -> number of commits with N parents`."] pub parent_counts : BTreeMap < u32 , u32 > , }
};
}

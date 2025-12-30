// Generated macro for Dfs (struct)
macro_rules! Depcrate_traversalsDfs {
() => {
// Module: crate::traversals
// Provides: {"Dfs"}
// Dependencies: {}
# [doc = " A depth-first traversal."] # [doc = ""] # [doc = " This is a fairly low-level traversal type, and is generally intended to be"] # [doc = " used as a building block for making specific pre-order or post-order"] # [doc = " traversals for whatever problem is at hand."] # [doc = ""] # [doc = " This type may be reused multiple times across different passes or functions"] # [doc = " and will internally reuse any heap allocations its already made."] # [doc = ""] # [doc = " Traversal is not recursive."] # [derive (Debug , Default , Clone)] pub struct Dfs { stack : Vec < (Event , ir :: Block) > , seen : EntitySet < ir :: Block > , }
};
}

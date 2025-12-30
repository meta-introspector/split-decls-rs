// Generated macro for Outcome (struct)
macro_rules! Depcrate_status_tree_indexOutcome {
() => {
// Module: crate::status::tree_index
// Provides: {"Outcome"}
// Dependencies: {}
# [doc = " The outcome of [Repository::tree_index_status()]."] # [derive (Clone)] pub struct Outcome { # [doc = " Additional information produced by the rename tracker."] # [doc = ""] # [doc = " It may be `None` if rename tracking was disabled."] pub rewrite : Option < gix_diff :: rewrites :: Outcome > , # [doc = " The index produced from the input `tree` for the purpose of diffing."] # [doc = ""] # [doc = " At some point this might go away once it's possible to diff an index from a tree directly."] pub tree_index : gix_index :: State , }
};
}

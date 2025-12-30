// Generated macro for CompletionRelevanceTraitInfo (struct)
macro_rules! Depcrate_itemCompletionRelevanceTraitInfo {
() => {
// Module: crate::item
// Provides: {"CompletionRelevanceTraitInfo"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , Eq , PartialEq)] pub struct CompletionRelevanceTraitInfo { # [doc = " The trait this item is from is a `#[doc(notable_trait)]`"] pub notable_trait : bool , # [doc = " Set for method completions of the `core::ops` and `core::cmp` family."] pub is_op_method : bool , }
};
}

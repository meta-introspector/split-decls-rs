// Generated macro for PathCheck (enum)
macro_rules! Depcrate_hir_utilsPathCheck {
() => {
// Module: crate::hir_utils
// Provides: {"PathCheck"}
// Dependencies: {}
# [doc = " Determines how paths are hashed and compared for equality."] # [derive (Copy , Clone , Debug , Default)] pub enum PathCheck { # [doc = " Paths must match exactly and are hashed by their exact HIR tree."] # [doc = ""] # [doc = " Thus, `std::iter::Iterator` and `Iterator` are not considered equal even though they refer"] # [doc = " to the same item."] # [default] Exact , # [doc = " Paths are compared and hashed based on their resolution."] # [doc = ""] # [doc = " They can appear different in the HIR tree but are still considered equal"] # [doc = " and have equal hashes as long as they refer to the same item."] # [doc = ""] # [doc = " Note that this is currently only partially implemented specifically for paths that are"] # [doc = " resolved before type-checking, i.e. the final segment must have a non-error resolution."] # [doc = " If a path with an error resolution is encountered, it falls back to the default exact"] # [doc = " matching behavior."] Resolution , }
};
}

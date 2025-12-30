// Generated macro for Source (enum)
macro_rules! Depcrate_stack_state_attributesSource {
() => {
// Module: crate::stack::state::attributes
// Provides: {"Source"}
// Dependencies: {}
# [doc = " Decide where to read `.gitattributes` files from."] # [doc = ""] # [doc = " To Retrieve attribute files from id mappings, see"] # [doc = " [State::id_mappings_from_index()][crate::stack::State::id_mappings_from_index()]."] # [doc = ""] # [doc = " These mappings are typically produced from an index."] # [doc = " If a tree should be the source, build an attribute list from a tree instead, or convert a tree to an index."] # [doc = ""] # [derive (Default , Debug , Clone , Copy)] pub enum Source { # [doc = " Use this when no worktree checkout is available, like in bare repositories, during clones, or when accessing blobs from"] # [doc = " other parts of the history which aren't checked out."] # [default] IdMapping , # [doc = " Read from an id mappings and if not present, read from the worktree."] # [doc = ""] # [doc = " This us typically used when *checking out* files."] IdMappingThenWorktree , # [doc = " Read from the worktree and if not present, read them from the id mappings."] # [doc = ""] # [doc = " This is typically used when *checking in* files, and it's possible for sparse worktrees not to have a `.gitattribute` file"] # [doc = " checked out even though it's available in the index."] WorktreeThenIdMapping , }
};
}

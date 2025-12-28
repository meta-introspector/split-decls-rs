macro_rules! Source {
    () => {
        # [doc = " Decide where to read `.gitignore` files from."] # [derive (Default , Debug , Clone , Copy)] pub enum Source { # [doc = " Retrieve ignore files from id mappings, see"] # [doc = " [State::id_mappings_from_index()][crate::stack::State::id_mappings_from_index()]."] # [doc = ""] # [doc = " These mappings are typically produced from an index."] # [doc = " If a tree should be the source, build an attribute list from a tree instead, or convert a tree to an index."] # [doc = ""] # [doc = " Use this when no worktree checkout is available, like in bare repositories or when accessing blobs from other parts"] # [doc = " of the history which aren't checked out."] IdMapping , # [doc = " Read from the worktree and if not present, read them from the id mappings *if* these don't have the skip-worktree bit set."] # [default] WorktreeThenIdMappingIfNotSkipped , }
    };
}

Source!()
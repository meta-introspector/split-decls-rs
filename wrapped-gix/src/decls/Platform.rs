macro_rules! deps {
    () => {
        Repository!();
        OwnedOrStaticAtomicBool!();
        IndexPersistedOrInMemory!();
        Options!();
        TrackRenames!();
        Submodule!();
    };
}

macro_rules! Platform {
    () => {
        deps!();
        # [doc = " A structure to hold options configuring the status request, which can then be turned into an iterator."] pub struct Platform < 'repo , Progress > where Progress : gix_features :: progress :: Progress + 'static , { repo : & 'repo Repository , progress : Progress , index : Option < crate :: worktree :: IndexPersistedOrInMemory > , head_tree : Option < Option < gix_hash :: ObjectId > > , submodules : Submodule , index_worktree_options : index_worktree :: Options , tree_index_renames : tree_index :: TrackRenames , should_interrupt : Option < OwnedOrStaticAtomicBool > , }
    };
}

Platform!();
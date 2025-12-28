macro_rules! deps {
    () => {
        Cache!();
        Options!();
        ThreadSafeRepository!();
        CommitsStorage!();
        OdbHandle!();
        Repository!();
        IndexStorage!();
        ModulesFileStorage!();
        RefStore!();
    };
}

macro_rules! impl_344 {
    () => {
        deps!();
        impl crate :: Repository { # [allow (clippy :: too_many_arguments)] pub (crate) fn from_refs_and_objects (refs : crate :: RefStore , mut objects : crate :: OdbHandle , work_tree : Option < std :: path :: PathBuf > , common_dir : Option < std :: path :: PathBuf > , config : crate :: config :: Cache , linked_worktree_options : crate :: open :: Options , # [cfg (feature = "index")] index : crate :: worktree :: IndexStorage , shallow_commits : crate :: shallow :: CommitsStorage , # [cfg (feature = "attributes")] modules : crate :: submodule :: ModulesFileStorage ,) -> Self { setup_objects (& mut objects , & config) ; crate :: Repository { bufs : Some (RefCell :: new (Vec :: with_capacity (4))) , work_tree , common_dir , objects , refs , config , options : linked_worktree_options , # [cfg (feature = "index")] index , shallow_commits , # [cfg (feature = "attributes")] modules , } } # [doc = " Convert this instance into a [`ThreadSafeRepository`][crate::ThreadSafeRepository] by dropping all thread-local data."] pub fn into_sync (self) -> crate :: ThreadSafeRepository { self . into () } }
    };
}

impl_344!();
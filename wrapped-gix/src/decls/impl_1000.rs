macro_rules! deps {
    () => {
        Error!();
        Submodule!();
        Note!();
        Platform!();
        Default!();
        Status!();
        Repository!();
        Options!();
    };
}

macro_rules! impl_1000 {
    () => {
        deps!();
        # [doc = " Status"] impl Repository { # [doc = " Obtain a platform for configuring iterators for traversing git repository status information."] # [doc = ""] # [doc = " By default, this is set to the fastest and most immediate way of obtaining a status,"] # [doc = " which is most similar to"] # [doc = ""] # [doc = " `git status --ignored=no`"] # [doc = ""] # [doc = " which implies that submodule information is provided by default."] # [doc = ""] # [doc = " Note that `status.showUntrackedFiles` is respected, which leads to untracked files being"] # [doc = " collapsed by default. If that needs to be controlled,"] # [doc = " [configure the directory walk explicitly](Platform::dirwalk_options) or more [implicitly](Platform::untracked_files)."] # [doc = ""] # [doc = " Pass `progress` to receive progress information on file modifications on this repository."] # [doc = " Use [`progress::Discard`](crate::progress::Discard) to discard all progress information."] # [doc = ""] # [doc = " ### Deviation"] # [doc = ""] # [doc = " Whereas Git runs the index-modified check before the directory walk to set entries"] # [doc = " as up-to-date to (potentially) safe some disk-access, we run both in parallel which"] # [doc = " ultimately is much faster."] pub fn status < P > (& self , progress : P) -> Result < Platform < '_ , P > , Error > where P : gix_features :: progress :: Progress + 'static , { let platform = Platform { repo : self , progress , index : None , submodules : Submodule :: default () , should_interrupt : None , head_tree : Some (None) , tree_index_renames : Default :: default () , index_worktree_options : index_worktree :: Options { sorting : None , dirwalk_options : Some (self . dirwalk_options () ?) , rewrites : None , thread_limit : None , } , } ; let untracked = self . config . resolved . string (config :: tree :: Status :: SHOW_UNTRACKED_FILES) . map (| value | { config :: tree :: Status :: SHOW_UNTRACKED_FILES . try_into_show_untracked_files (value) . with_lenient_default (self . config . lenient_config) }) . transpose () ? . unwrap_or_default () ; Ok (platform . untracked_files (untracked)) } }
    };
}

impl_1000!()
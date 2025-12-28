macro_rules! deps {
    () => {
        Default!();
        Outcome!();
        Error!();
        Pathspec!();
        Action!();
        Status!();
        Repository!();
        TrackRenames!();
        State!();
    };
}

macro_rules! impl_1018 {
    () => {
        deps!();
        impl Repository { # [doc = " Produce the `git status` portion that shows the difference between `tree_id` (usually `HEAD^{tree}`) and the `worktree_index`"] # [doc = " (typically the current `.git/index`), and pass all changes to `cb(change, tree_index, worktree_index)` with"] # [doc = " full access to both indices that contributed to the change."] # [doc = ""] # [doc = " *(It's notable that internally, the `tree_id` is converted into an index before diffing these)*."] # [doc = " Set `pathspec` to `Some(_)` to further reduce the set of files to check."] # [doc = ""] # [doc = " ### Notes"] # [doc = ""] # [doc = " * This is a low-level method - prefer the [`Repository::status()`] platform instead for access to various iterators"] # [doc = "   over the same information."] pub fn tree_index_status < 'repo , E > (& 'repo self , tree_id : & gix_hash :: oid , worktree_index : & gix_index :: State , pathspec : Option < & mut crate :: Pathspec < 'repo > > , renames : TrackRenames , mut cb : impl FnMut (gix_diff :: index :: ChangeRef < '_ , '_ > , & gix_index :: State , & gix_index :: State ,) -> Result < gix_diff :: index :: Action , E > ,) -> Result < Outcome , Error > where E : Into < Box < dyn std :: error :: Error + Send + Sync > > , { let _span = gix_trace :: coarse ! ("gix::tree_index_status") ; let tree_index : gix_index :: State = self . index_from_tree (tree_id) ? . into () ; let rewrites = match renames { TrackRenames :: AsConfigured => { let (mut rewrites , mut is_configured) = crate :: diff :: utils :: new_rewrites_inner (& self . config . resolved , self . config . lenient_config , & tree :: Status :: RENAMES , & tree :: Status :: RENAME_LIMIT ,) ? ; if ! is_configured { (rewrites , is_configured) = crate :: diff :: utils :: new_rewrites (& self . config . resolved , self . config . lenient_config) ? ; } if ! is_configured { rewrites = Some (Default :: default ()) ; } rewrites } TrackRenames :: Given (rewrites) => Some (rewrites) , TrackRenames :: Disabled => None , } ; let mut resource_cache = None ; if rewrites . is_some () { resource_cache = Some (self . diff_resource_cache_for_tree_diff () ?) ; } let mut pathspec_storage = None ; if pathspec . is_none () { pathspec_storage = self . pathspec (true , None :: < & str > , false , & gix_index :: State :: new (self . object_hash ()) , gix_worktree :: stack :: state :: attributes :: Source :: IdMapping ,) . expect ("Impossible for this to fail without patterns") . into () ; } let pathspec = pathspec . unwrap_or_else (| | pathspec_storage . as_mut () . expect ("set if pathspec isn't set by user")) ; let rewrite = gix_diff :: index (& tree_index , worktree_index , | change | cb (change , & tree_index , worktree_index) , rewrites . zip (resource_cache . as_mut ()) . map (| (rewrites , resource_cache) | gix_diff :: index :: RewriteOptions { resource_cache , find : self , rewrites , }) , & mut pathspec . search , & mut | relative_path , case , is_dir , out | { let stack = pathspec . stack . as_mut () . expect ("initialized in advance") ; stack . set_case (case) . at_entry (relative_path , Some (crate :: pathspec :: is_dir_to_mode (is_dir)) , & pathspec . repo . objects ,) . is_ok_and (| platform | platform . matching_attributes (out)) } ,) ? ; Ok (Outcome { rewrite , tree_index }) } }
    };
}

impl_1018!()
macro_rules! deps {
    () => {
        Pattern!();
        Error!();
        Search!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        # [doc = " Lifecycle"] impl Search { # [doc = " Create a search from ready-made `pathspecs`, and [normalize](Pattern::normalize()) them with `prefix` and `root`."] # [doc = " `root` is the absolute path to the worktree root, if available, or the `git_dir` in case of bare repositories."] # [doc = " If `pathspecs` doesn't yield any pattern, we will match everything automatically. If `prefix` is also provided and not empty,"] # [doc = " an artificial pattern will be added to yield all."] pub fn from_specs (pathspecs : impl IntoIterator < Item = Pattern > , prefix : Option < & std :: path :: Path > , root : & std :: path :: Path ,) -> Result < Self , crate :: normalize :: Error > { fn inner (pathspecs : & mut dyn Iterator < Item = Pattern > , prefix : Option < & std :: path :: Path > , root : & std :: path :: Path ,) -> Result < Search , crate :: normalize :: Error > { let prefix = prefix . unwrap_or (std :: path :: Path :: new ("")) ; let mut patterns = pathspecs . enumerate () . map (| (idx , pattern) | mapping_from_pattern (pattern , prefix , root , idx)) . collect :: < Result < Vec < _ > , _ > > () ? ; if patterns . is_empty () && ! prefix . as_os_str () . is_empty () { patterns . push (mapping_from_pattern (Pattern :: from_literal (& [] , MagicSignature :: MUST_BE_DIR) , prefix , root , 0 ,) ?) ; } patterns . sort_by (| a , b | { a . value . pattern . is_excluded () . cmp (& b . value . pattern . is_excluded ()) . reverse () }) ; let common_prefix_len = common_prefix_len (& patterns) ; Ok (Search { all_patterns_are_excluded : patterns . iter () . all (| s | s . value . pattern . is_excluded ()) , patterns , source : None , common_prefix_len , }) } inner (& mut pathspecs . into_iter () , prefix , root) } # [doc = " Obtain ownership of the normalized pathspec patterns that were used for the search."] pub fn into_patterns (self) -> impl Iterator < Item = Pattern > { self . patterns . into_iter () . map (| p | p . value . pattern) } }
    };
}

impl_10!();
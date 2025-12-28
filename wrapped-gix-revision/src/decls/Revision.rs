macro_rules! deps {
    () => {
        SiblingBranch!();
        PrefixHint!();
        ReflogLookup!();
    };
}

macro_rules! Revision {
    () => {
        deps!();
        # [doc = " Usually the first methods to call when parsing a rev-spec to set an anchoring revision (which is typically a `Commit` object)."] # [doc = " Methods can be called multiple time to either try input or to parse another rev-spec that is part of a range."] # [doc = ""] # [doc = " In one case they will not be called at all, e.g. `@{[-]n}` indicates the current branch (what `HEAD` dereferences to),"] # [doc = " without ever naming it, and so does `@{upstream}` or `@{<date>}`."] # [doc = ""] # [doc = " Note that when dereferencing `HEAD` implicitly, a revision must be set for later navigation."] pub trait Revision { # [doc = " Resolve `name` as reference which might not be a valid reference name. The name may be partial like `main` or full like"] # [doc = " `refs/heads/main` solely depending on the users input."] # [doc = " Symbolic referenced should be followed till their object, but objects **must not yet** be peeled."] fn find_ref (& mut self , name : & BStr) -> Option < () > ; # [doc = " An object prefix to disambiguate, returning `None` if it is ambiguous or wasn't found at all."] # [doc = ""] # [doc = " If `hint` is set, it should be used to disambiguate multiple objects with the same prefix."] fn disambiguate_prefix (& mut self , prefix : gix_hash :: Prefix , hint : Option < PrefixHint < '_ > >) -> Option < () > ; # [doc = " Lookup the reflog of the previously set reference, or dereference `HEAD` to its reference"] # [doc = " to obtain the ref name (as opposed to `HEAD` itself)."] # [doc = " If there is no such reflog entry, return `None`."] fn reflog (& mut self , query : ReflogLookup) -> Option < () > ; # [doc = " When looking at `HEAD`, `branch_no` is the non-null checkout in the path, e.g. `1` means the last branch checked out,"] # [doc = " `2` is the one before that."] # [doc = " Return `None` if there is no branch as the checkout history (via the reflog) isn't long enough."] fn nth_checked_out_branch (& mut self , branch_no : usize) -> Option < () > ; # [doc = " Lookup the previously set branch or dereference `HEAD` to its reference to use its name to lookup the sibling branch of `kind`"] # [doc = " in the configuration (typically in `refs/remotes/…`). The sibling branches are always local tracking branches."] # [doc = " Return `None` of no such configuration exists and no sibling could be found, which is also the case for all reference outside"] # [doc = " of `refs/heads/`."] # [doc = " Note that the caller isn't aware if the previously set reference is a branch or not and might call this method even though no reference"] # [doc = " is known."] fn sibling_branch (& mut self , kind : SiblingBranch) -> Option < () > ; }
    };
}

Revision!()
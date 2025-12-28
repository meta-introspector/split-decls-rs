macro_rules! deps {
    () => {
        Worktree!();
        Error!();
        Item!();
        Note!();
        Pathspec!();
        Init!();
        ApplyLeniencyDefaultValue!();
    };
}

macro_rules! pathspec {
    () => {
        deps!();
        # [doc = ""] # [cfg (feature = "attributes")] pub mod pathspec { use crate :: { bstr :: BStr , config :: { cache :: util :: ApplyLeniencyDefaultValue , tree :: gitoxide } , Worktree , } ; # [doc = " The error returned by [`Worktree::pathspec()`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Init (# [from] crate :: pathspec :: init :: Error) , # [error (transparent)] OpenIndex (# [from] crate :: worktree :: open_index :: Error) , } impl < 'repo > Worktree < 'repo > { # [doc = " Configure pathspecs `patterns` to be matched against, with pathspec attributes read from the worktree and then from the index"] # [doc = " if needed."] # [doc = ""] # [doc = " Note that the `empty_patterns_match_prefix` flag of the [parent method](crate::Repository::pathspec()) defaults to `true`."] # [doc = ""] # [doc = " ### Deviation"] # [doc = ""] # [doc = " Pathspec attributes match case-insensitively by default if the underlying filesystem is configured that way."] pub fn pathspec (& self , patterns : impl IntoIterator < Item = impl AsRef < BStr > > ,) -> Result < crate :: Pathspec < 'repo > , Error > { let index = self . index () ? ; let inherit_ignore_case = self . parent . config . resolved . boolean ("gitoxide.pathspec.inheritIgnoreCase") . map (| res | { gitoxide :: Pathspec :: INHERIT_IGNORE_CASE . enrich_error (res) . with_lenient_default_value (self . parent . config . lenient_config , gitoxide :: Pathspec :: INHERIT_IGNORE_CASE_DEFAULT ,) }) . transpose () . map_err (| err | Error :: Init (crate :: pathspec :: init :: Error :: Defaults (err . into ()))) ? . unwrap_or (gitoxide :: Pathspec :: INHERIT_IGNORE_CASE_DEFAULT) ; Ok (self . parent . pathspec (true , patterns , inherit_ignore_case , & index , gix_worktree :: stack :: state :: attributes :: Source :: WorktreeThenIdMapping ,) ?) } } }
    };
}

pathspec!()
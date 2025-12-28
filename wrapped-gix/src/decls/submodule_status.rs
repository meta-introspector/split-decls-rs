macro_rules! deps {
    () => {
        Status!();
        Diff!();
        ApplyLeniency!();
        Entry!();
        Submodule!();
        ThreadSafeRepository!();
        BuiltinSubmoduleStatus!();
        Error!();
    };
}

macro_rules! submodule_status {
    () => {
        deps!();
        # [doc = ""] mod submodule_status { use std :: borrow :: Cow ; use crate :: config :: cache :: util :: ApplyLeniency ; use crate :: { bstr , bstr :: BStr , config , status :: { index_worktree :: BuiltinSubmoduleStatus , Submodule } , } ; impl BuiltinSubmoduleStatus { # [doc = " Create a new instance from a `repo` and a `mode` to control how the submodule status will be obtained."] pub fn new (repo : crate :: ThreadSafeRepository , mode : Submodule ,) -> Result < Self , crate :: submodule :: modules :: Error > { let local_repo = repo . to_thread_local () ; let submodule_paths = match local_repo . submodules () { Ok (Some (sm)) => { let mut v : Vec < _ > = sm . filter_map (| sm | sm . path () . ok () . map (Cow :: into_owned)) . collect () ; v . sort () ; v } Ok (None) => Vec :: new () , Err (err) => return Err (err) , } ; Ok (Self { mode , # [cfg (feature = "parallel")] repo , # [cfg (not (feature = "parallel"))] git_dir : local_repo . git_dir () . to_owned () , submodule_paths , }) } } # [doc = " The error returned submodule status checks."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] SubmoduleStatus (# [from] crate :: submodule :: status :: Error) , # [error (transparent)] IgnoreConfig (# [from] crate :: submodule :: config :: Error) , # [error (transparent)] DiffSubmoduleIgnoreConfig (# [from] config :: key :: GenericErrorWithValue) , } impl gix_status :: index_as_worktree :: traits :: SubmoduleStatus for BuiltinSubmoduleStatus { type Output = crate :: submodule :: Status ; type Error = Error ; fn status (& mut self , _entry : & gix_index :: Entry , rela_path : & BStr) -> Result < Option < Self :: Output > , Self :: Error > { use bstr :: ByteSlice ; if self . submodule_paths . binary_search_by (| path | path . as_bstr () . cmp (rela_path)) . is_err () { return Ok (None) ; } # [cfg (feature = "parallel")] let repo = self . repo . to_thread_local () ; # [cfg (not (feature = "parallel"))] let Ok (repo) = crate :: open (& self . git_dir) else { return Ok (None) ; } ; let Ok (Some (mut submodules)) = repo . submodules () else { return Ok (None) ; } ; let Some (sm) = submodules . find (| sm | sm . path () . is_ok_and (| path | path == rela_path)) else { return Ok (None) ; } ; let (ignore , check_dirty) = match self . mode { Submodule :: AsConfigured { check_dirty } => { let global_ignore = repo . config_snapshot () . string (& config :: tree :: Diff :: IGNORE_SUBMODULES) . map (| value | config :: tree :: Diff :: IGNORE_SUBMODULES . try_into_ignore (value)) . transpose () . with_leniency (repo . config . lenient_config) ? ; if let Some (ignore) = global_ignore { (ignore , check_dirty) } else { let ignore = sm . ignore () ? . unwrap_or_default () ; (ignore , check_dirty) } } Submodule :: Given { ignore , check_dirty } => (ignore , check_dirty) , } ; let status = sm . status (ignore , check_dirty) ? ; Ok (status . is_dirty () . and_then (| dirty | dirty . then_some (status))) } } }
    };
}

submodule_status!();
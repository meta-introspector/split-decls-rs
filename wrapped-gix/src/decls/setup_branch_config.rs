macro_rules! deps {
    () => {
        Error!();
        Repository!();
        Item!();
        WriteMode!();
    };
}

macro_rules! setup_branch_config {
    () => {
        deps!();
        # [doc = " Set up the remote configuration for `branch` so that it points to itself, but on the remote, if and only if currently"] # [doc = " saved refspecs are able to match it."] # [doc = " For that we reload the remote of `remote_name` and use its `ref_specs` for match."] fn setup_branch_config (repo : & mut Repository , branch : & FullNameRef , branch_id : Option < & gix_hash :: oid > , remote_name : & BStr ,) -> Result < () , Error > { let short_name = match branch . category_and_short_name () { Some ((gix_ref :: Category :: LocalBranch , shortened)) => match shortened . to_str () { Ok (s) => s , Err (_) => return Ok (()) , } , _ => return Ok (()) , } ; let remote = repo . find_remote (remote_name) . expect ("remote was just created and must be visible in config") ; let group = gix_refspec :: MatchGroup :: from_fetch_specs (remote . fetch_specs . iter () . map (gix_refspec :: RefSpec :: to_ref)) ; let null = gix_hash :: ObjectId :: null (repo . object_hash ()) ; let res = group . match_lhs (Some (gix_refspec :: match_group :: Item { full_ref_name : branch . as_bstr () , target : branch_id . unwrap_or (& null) , object : None , }) . into_iter () ,) ; if ! res . mappings . is_empty () { let mut config = repo . config_snapshot_mut () ; let mut section = config . new_section ("branch" , Some (Cow :: Owned (short_name . into ()))) . expect ("section header name is always valid per naming rules, our input branch name is valid") ; section . push ("remote" . try_into () . expect ("valid at compile time") , Some (remote_name)) ; section . push ("merge" . try_into () . expect ("valid at compile time") , Some (branch . as_bstr ()) ,) ; write_to_local_config (& config , WriteMode :: Overwrite) ? ; config . commit () . expect ("configuration we set is valid") ; } Ok (()) }
    };
}

setup_branch_config!();
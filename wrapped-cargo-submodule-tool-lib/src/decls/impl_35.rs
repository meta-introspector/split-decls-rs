macro_rules! deps {
    () => {
        RealCargoTomlProcessor!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        # [cfg (feature = "toml_edit_enabled")] impl RealCargoTomlProcessor { fn clean_repo_url (& self , repo_url : & str) -> String { let regex_matcher = CurrentRegexMatcher :: new (r"^(https?://github\.com/[^/]+/[^/.]+)(/tree/[^/]+/.+)?(\.git)?$" ,) . expect ("Failed to create regex matcher for clean_repo_url") ; let x = if let Some (captures) = regex_matcher . captures (repo_url) { let base_url = captures . get (1) . expect ("Expected capture group 1") . to_string () ; let git_suffix = captures . get (3) . unwrap_or ("") . to_string () ; format ! ("{}{}" , base_url , git_suffix) } else { repo_url . to_string () } ; x } fn extract_owner_repo_name (& self , cleaned_url : & str) -> Option < (String , String) > { let regex_matcher = CurrentRegexMatcher :: new (r"github\.com/([^/]+)/([^/.]+)(\.git)?") . expect ("Failed to create regex matcher for extract_owner_repo_name") ; let x = if let Some (captures) = regex_matcher . captures (cleaned_url) { let owner = captures . get (1) . expect ("Expected capture group 1") . to_string () ; let repo_name = captures . get (2) . expect ("Expected capture group 2") . to_string () ; Some ((owner , repo_name)) } else { None } ; x } }
    };
}

impl_35!()
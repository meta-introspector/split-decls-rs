macro_rules! deps {
    () => {
        Error!();
        Remote!();
        Repository!();
        Default!();
        Url!();
    };
}

macro_rules! impl_922 {
    () => {
        deps!();
        # [doc = " Initialization"] impl < 'repo > Remote < 'repo > { # [allow (clippy :: too_many_arguments)] pub (crate) fn from_preparsed_config (name_or_url : Option < BString > , url : Option < gix_url :: Url > , push_url : Option < gix_url :: Url > , fetch_specs : Vec < RefSpec > , push_specs : Vec < RefSpec > , should_rewrite_urls : bool , fetch_tags : remote :: fetch :: Tags , repo : & 'repo Repository ,) -> Result < Self , Error > { debug_assert ! (url . is_some () || push_url . is_some () , "BUG: fetch or push url must be set at least") ; let (url_alias , push_url_alias) = if should_rewrite_urls { rewrite_urls (& repo . config , url . as_ref () , push_url . as_ref ()) } else { Ok ((None , None)) } ? ; Ok (Remote { name : name_or_url . map (Into :: into) , url , url_alias , push_url , push_url_alias , fetch_specs , push_specs , fetch_tags , repo , }) } pub (crate) fn from_fetch_url < Url , E > (url : Url , should_rewrite_urls : bool , repo : & 'repo Repository ,) -> Result < Self , Error > where Url : TryInto < gix_url :: Url , Error = E > , gix_url :: parse :: Error : From < E > , { Self :: from_fetch_url_inner (url . try_into () . map_err (| err | Error :: Url (err . into ())) ? , should_rewrite_urls , repo ,) } fn from_fetch_url_inner (url : gix_url :: Url , should_rewrite_urls : bool , repo : & 'repo Repository ,) -> Result < Self , Error > { let (url_alias , _) = if should_rewrite_urls { rewrite_urls (& repo . config , Some (& url) , None) } else { Ok ((None , None)) } ? ; Ok (Remote { name : None , url : Some (url) , url_alias , push_url : None , push_url_alias : None , fetch_specs : Vec :: new () , push_specs : Vec :: new () , fetch_tags : Default :: default () , repo , }) } }
    };
}

impl_922!()
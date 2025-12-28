macro_rules! deps {
    () => {
        Rewrite!();
        Replace!();
        Url!();
        Init!();
    };
}

macro_rules! impl_977 {
    () => {
        deps!();
        # [doc = " Init"] impl Rewrite { pub fn from_config (config : & gix_config :: File < 'static > , mut filter : fn (& gix_config :: file :: Metadata) -> bool ,) -> Rewrite { config . sections_by_name_and_filter ("url" , & mut filter) . map (| sections | { let mut url_rewrite = Vec :: new () ; let mut push_url_rewrite = Vec :: new () ; for section in sections { let replace = match section . header () . subsection_name () { Some (base) => OwnShared :: new (base . to_owned ()) , None => continue , } ; for instead_of in section . values (config :: tree :: Url :: INSTEAD_OF . name) { url_rewrite . push (Replace { with : OwnShared :: clone (& replace) , find : instead_of . into_owned () , }) ; } for instead_of in section . values (config :: tree :: Url :: PUSH_INSTEAD_OF . name) { push_url_rewrite . push (Replace { with : OwnShared :: clone (& replace) , find : instead_of . into_owned () , }) ; } } Rewrite { url_rewrite , push_url_rewrite , } }) . unwrap_or_default () } }
    };
}

impl_977!();
macro_rules! deps {
    () => {
        Error!();
        Direction!();
        Push!();
        Cache!();
        Url!();
        Fetch!();
    };
}

macro_rules! rewrite_urls {
    () => {
        deps!();
        pub (crate) fn rewrite_urls (config : & config :: Cache , url : Option < & gix_url :: Url > , push_url : Option < & gix_url :: Url > ,) -> Result < (Option < gix_url :: Url > , Option < gix_url :: Url >) , Error > { let url_alias = rewrite_url (config , url , remote :: Direction :: Fetch) ? ; let push_url_alias = rewrite_url (config , push_url , remote :: Direction :: Push) ? ; Ok ((url_alias , push_url_alias)) }
    };
}

rewrite_urls!();
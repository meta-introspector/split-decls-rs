// Generated macro for rewrite_urls (function)
macro_rules! Depcrate_remote_initrewrite_urls {
() => {
// Module: crate::remote::init
// Provides: {"rewrite_urls"}
// Dependencies: {}
pub (crate) fn rewrite_urls (config : & config :: Cache , url : Option < & gix_url :: Url > , push_url : Option < & gix_url :: Url > ,) -> Result < (Option < gix_url :: Url > , Option < gix_url :: Url >) , Error > { let url_alias = rewrite_url (config , url , remote :: Direction :: Fetch) ? ; let push_url_alias = rewrite_url (config , push_url , remote :: Direction :: Push) ? ; Ok ((url_alias , push_url_alias)) }
};
}

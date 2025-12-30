// Generated macro for tests (module)
macro_rules! Depcrate_client_blocking_io_http_redirecttests {
() => {
// Module: crate::client::blocking_io::http::redirect
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn base_url_complete () { assert_eq ! (base_url ("https://redirected.org/b/info/refs?hi" , "https://original/a" , "https://original/a/info/refs?hi" . into ()) . unwrap () , "https://redirected.org/b") ; } # [test] fn swap_tails_complete () { assert_eq ! (swap_tails (None , "not interesting" , "used" . into ()) , "used" , "without effective base url, it passes url, no redirect happened yet") ; assert_eq ! (swap_tails (Some ("https://redirected.org/b") , "https://original/a" , "https://original/a/info/refs?something" . into ()) , "https://redirected.org/b/info/refs?something" , "the tail stays the same if redirection happened") ; } }
};
}

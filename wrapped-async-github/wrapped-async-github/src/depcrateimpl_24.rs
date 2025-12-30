// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl From < & OctoPullRequest > for PullRequest { fn from (pr : & OctoPullRequest) -> Self { Self { id : pr . number . to_string () , title : pr . title . as_ref () . unwrap () . to_string () , url : pr . html_url . as_ref () . map (ToString :: to_string) . unwrap_or_default () , } } }
};
}

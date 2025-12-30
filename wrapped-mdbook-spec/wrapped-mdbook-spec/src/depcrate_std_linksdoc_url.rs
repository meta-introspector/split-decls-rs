// Generated macro for DOC_URL (static)
macro_rules! Depcrate_std_linksDOC_URL {
() => {
// Module: crate::std_links
// Provides: {"DOC_URL"}
// Dependencies: {}
static DOC_URL : Lazy < Regex > = Lazy :: new (| | { Regex :: new (r"^https://doc.rust-lang.org/(?:nightly|beta|stable|dev|1\.[0-9]+\.[0-9]+)") . unwrap () }) ;
};
}

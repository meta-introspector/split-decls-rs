// Generated macro for ANCHOR_URL (static)
macro_rules! Depcrate_std_linksANCHOR_URL {
() => {
// Module: crate::std_links
// Provides: {"ANCHOR_URL"}
// Dependencies: {}
# [doc = " The Regex used to extract the URL from an HTML link."] static ANCHOR_URL : Lazy < Regex > = Lazy :: new (| | Regex :: new ("<a href=\"([^\"]+)\"") . unwrap ()) ;
};
}

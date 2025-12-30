// Generated macro for is_html_tag (function)
macro_rules! Depcrate_scannersis_html_tag {
() => {
// Module: crate::scanners
// Provides: {"is_html_tag"}
// Dependencies: {}
fn is_html_tag (tag : & [u8]) -> bool { HTML_TAGS . binary_search_by (| probe | { let probe_bytes_iter = probe . as_bytes () . iter () ; let tag_bytes_iter = tag . iter () ; probe_bytes_iter . zip (tag_bytes_iter) . find_map (| (& a , & b) | { match a . cmp (& (b | 0x20)) { core :: cmp :: Ordering :: Equal => None , inequality => Some (inequality) , } }) . unwrap_or_else (| | probe . len () . cmp (& tag . len ())) }) . is_ok () }
};
}

// Generated macro for extract_simple_path (function)
macro_rules! Depcrate_utilextract_simple_path {
() => {
// Module: crate::util
// Provides: {"extract_simple_path"}
// Dependencies: {}
# [doc = " Extracts a simple non-global path of length 1."] pub fn extract_simple_path (path : & syn :: Path) -> Option < & syn :: Ident > { match_singleton (& path . segments) . filter (| f | ! path_is_global (path) && f . arguments . is_empty ()) . map (| f | & f . ident) }
};
}

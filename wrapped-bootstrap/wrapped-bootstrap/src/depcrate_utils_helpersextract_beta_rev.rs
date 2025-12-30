// Generated macro for extract_beta_rev (function)
macro_rules! Depcrate_utils_helpersextract_beta_rev {
() => {
// Module: crate::utils::helpers
// Provides: {"extract_beta_rev"}
// Dependencies: {}
# [doc = " Extract the beta revision from the full version string."] # [doc = ""] # [doc = " The full version string looks like \"a.b.c-beta.y\". And we need to extract"] # [doc = " the \"y\" part from the string."] pub fn extract_beta_rev (version : & str) -> Option < String > { let parts = version . splitn (2 , "-beta.") . collect :: < Vec < _ > > () ; parts . get (1) . and_then (| s | s . find (' ') . map (| p | s [.. p] . to_string ())) }
};
}

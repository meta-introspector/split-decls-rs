// Generated macro for determine_visibility (function)
macro_rules! Depcrate_utilsdetermine_visibility {
() => {
// Module: crate::utils
// Provides: {"determine_visibility"}
// Dependencies: {}
# [doc = " Determines the visibility of the projected types and projection methods."] # [doc = ""] # [doc = " If given visibility is `pub`, returned visibility is `pub(crate)`."] # [doc = " Otherwise, returned visibility is the same as given visibility."] pub (crate) fn determine_visibility (vis : & Visibility) -> Visibility { if let Visibility :: Public (token) = vis { parse_quote_spanned ! (token . span => pub (crate)) } else { vis . clone () } }
};
}

// Generated macro for in_options (function)
macro_rules! Depcrate_hbsin_options {
() => {
// Module: crate::hbs
// Provides: {"in_options"}
// Dependencies: {}
# [doc = " Whether or not the context is currently inside a `{{#options}}` block."] fn in_options (rc : & RenderContext < '_ , '_ >) -> bool { rc . context () . map_or (false , | ctx | ctx . data () . get ("__MDMAN_IN_OPTIONS") . is_some ()) }
};
}

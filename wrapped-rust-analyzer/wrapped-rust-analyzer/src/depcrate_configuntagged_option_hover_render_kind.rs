// Generated macro for untagged_option_hover_render_kind (function)
macro_rules! Depcrate_configuntagged_option_hover_render_kind {
() => {
// Module: crate::config
// Provides: {"untagged_option_hover_render_kind"}
// Dependencies: {}
# [test] fn untagged_option_hover_render_kind () { let hex = MemoryLayoutHoverRenderKindDef :: Hexadecimal ; let ser = serde_json :: to_string (& Some (hex)) . unwrap () ; assert_eq ! (& ser , "\"hexadecimal\"") ; let opt : Option < _ > = serde_json :: from_str ("\"hexadecimal\"") . unwrap () ; assert_eq ! (opt , Some (hex)) ; }
};
}

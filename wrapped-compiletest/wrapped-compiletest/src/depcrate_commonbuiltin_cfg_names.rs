// Generated macro for builtin_cfg_names (function)
macro_rules! Depcrate_commonbuiltin_cfg_names {
() => {
// Module: crate::common
// Provides: {"builtin_cfg_names"}
// Dependencies: {}
fn builtin_cfg_names (config : & Config) -> HashSet < String > { query_rustc_output (config , & ["--print=check-cfg" , "-Zunstable-options" , "--check-cfg=cfg()"] , Default :: default () ,) . lines () . map (| l | if let Some ((name , _)) = l . split_once ('=') { name . to_string () } else { l . to_string () }) . chain (std :: iter :: once (String :: from ("test"))) . collect () }
};
}

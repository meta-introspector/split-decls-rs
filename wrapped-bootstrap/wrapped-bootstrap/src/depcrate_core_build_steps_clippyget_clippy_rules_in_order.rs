// Generated macro for get_clippy_rules_in_order (function)
macro_rules! Depcrate_core_build_steps_clippyget_clippy_rules_in_order {
() => {
// Module: crate::core::build_steps::clippy
// Provides: {"get_clippy_rules_in_order"}
// Dependencies: {}
# [doc = " We need to keep the order of the given clippy lint rules before passing them."] # [doc = " Since clap doesn't offer any useful interface for this purpose out of the box,"] # [doc = " we have to handle it manually."] pub fn get_clippy_rules_in_order (all_args : & [String] , config : & LintConfig) -> Vec < String > { let mut result = vec ! [] ; for (prefix , item) in [("-A" , & config . allow) , ("-D" , & config . deny) , ("-W" , & config . warn) , ("-F" , & config . forbid)] { item . iter () . for_each (| v | { let rule = format ! ("{prefix}{v}") ; let position = all_args . iter () . position (| t | t == & rule || t == v) . unwrap_or (usize :: MAX) ; result . push ((position , rule)) ; }) ; } result . sort_by_key (| & (position , _) | position) ; result . into_iter () . map (| v | v . 1) . collect () }
};
}

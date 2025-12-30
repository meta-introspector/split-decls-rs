// Generated macro for term_width (function)
macro_rules! Depcrate_utilsterm_width {
() => {
// Module: crate::utils
// Provides: {"term_width"}
// Dependencies: {}
# [doc = " Returns the term width that insta should use."] pub fn term_width () -> usize { # [cfg (feature = "colors")] { console :: Term :: stdout () . size () . 1 as usize } # [cfg (not (feature = "colors"))] { 74 } }
};
}

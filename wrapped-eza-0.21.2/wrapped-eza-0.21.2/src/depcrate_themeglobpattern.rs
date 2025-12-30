// Generated macro for GlobPattern (enum)
macro_rules! Depcrate_themeGlobPattern {
() => {
// Module: crate::theme
// Provides: {"GlobPattern"}
// Dependencies: {}
# [derive (PartialEq , Debug)] # [doc = " Using a hashmap here for \"simple\" patterns (plain extensions like '*.txt')"] # [doc = " improves performance drastically for complex `LS_COLORS` usage (see"] # [doc = " <https://github.com/eza-community/eza/pull/1421#issuecomment-2816666661>)."] # [doc = ""] # [doc = " It doesn't change highlighting behavior, as we still walk the"] # [doc = " [`ExtensionMappings`] in reverse order, and the hashmap will only consist of"] # [doc = " disjoint sets (it doesn't matter in which order we search *.txt or *.pdf)."] # [doc = ""] # [doc = " In the event that a pattern shows up twice, we will use the later one (since"] # [doc = " .insert overrides any entry that exists), which is the correct behavior."] enum GlobPattern { Complex (glob :: Pattern , Style) , Simple (HashMap < String , Style >) , }
};
}

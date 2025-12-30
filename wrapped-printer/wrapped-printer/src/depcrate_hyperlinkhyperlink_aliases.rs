// Generated macro for hyperlink_aliases (function)
macro_rules! Depcrate_hyperlinkhyperlink_aliases {
() => {
// Module: crate::hyperlink
// Provides: {"hyperlink_aliases"}
// Dependencies: {}
# [doc = " Returns the set of hyperlink aliases supported by this crate."] # [doc = ""] # [doc = " Aliases are supported by the `FromStr` trait implementation of a"] # [doc = " [`HyperlinkFormat`]. That is, if an alias is seen, then it is automatically"] # [doc = " replaced with the corresponding format. For example, the `vscode` alias"] # [doc = " maps to `vscode://file{path}:{line}:{column}`."] # [doc = ""] # [doc = " This is exposed to allow callers to include hyperlink aliases in"] # [doc = " documentation in a way that is guaranteed to match what is actually"] # [doc = " supported."] # [doc = ""] # [doc = " The list returned is guaranteed to be sorted lexicographically"] # [doc = " by the alias name. Callers may want to re-sort the list using"] # [doc = " [`HyperlinkAlias::display_priority`] via a stable sort when showing the"] # [doc = " list to users. This will cause special aliases like `none` and `default` to"] # [doc = " appear first."] pub fn hyperlink_aliases () -> Vec < HyperlinkAlias > { HYPERLINK_PATTERN_ALIASES . iter () . cloned () . collect () }
};
}

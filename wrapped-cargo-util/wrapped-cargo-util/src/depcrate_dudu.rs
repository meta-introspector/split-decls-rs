// Generated macro for du (function)
macro_rules! Depcrate_dudu {
() => {
// Module: crate::du
// Provides: {"du"}
// Dependencies: {}
# [doc = " Determines the disk usage of all files in the given directory."] # [doc = ""] # [doc = " The given patterns are gitignore style patterns relative to the given"] # [doc = " path. If there are patterns, it will only count things matching that"] # [doc = " pattern. `!` can be used to exclude things. See [`OverrideBuilder::add`]"] # [doc = " for more info."] # [doc = ""] # [doc = " This is a primitive implementation that doesn't handle hard links, and"] # [doc = " isn't particularly fast (for example, not using `getattrlistbulk` on"] # [doc = " macOS). It also only uses actual byte sizes instead of block counts (and"] # [doc = " thus vastly undercounts directories with lots of small files). It would be"] # [doc = " nice to improve this or replace it with something better."] pub fn du (path : & Path , patterns : & [& str]) -> Result < u64 > { du_inner (path , patterns) . with_context (| | format ! ("failed to walk `{}`" , path . display ())) }
};
}

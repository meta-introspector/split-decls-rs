// Generated macro for UsageCache (struct)
macro_rules! Depcrate_searchUsageCache {
() => {
// Module: crate::search
// Provides: {"UsageCache"}
// Dependencies: {}
# [doc = " A cache for the results of find_usages. This is for when we have multiple patterns that have the"] # [doc = " same path. e.g. if the pattern was `foo::Bar` that can parse as a path, an expression, a type"] # [doc = " and as a pattern. In each, the usages of `foo::Bar` are the same and we'd like to avoid finding"] # [doc = " them more than once."] # [derive (Default)] pub (crate) struct UsageCache { usages : Vec < (Definition , UsageSearchResult) > , }
};
}

// Generated macro for PatternItem (enum)
macro_rules! Depcrate_commonPatternItem {
() => {
// Module: crate::common
// Provides: {"PatternItem"}
// Dependencies: {}
# [doc = " A borrowed item in a [`Pattern`]. Items are either string literals or placeholders."] # [doc = ""] # [doc = " [`Pattern`]: crate::Pattern"] # [derive (Debug , Copy , Clone , PartialEq , Eq , PartialOrd , Ord)] # [allow (clippy :: exhaustive_enums)] pub enum PatternItem < 'a , T > { # [doc = " A placeholder of the type specified on this [`PatternItem`]."] Placeholder (T) , # [doc = " A string literal. This can occur in one of three places:"] # [doc = ""] # [doc = " 1. Between the start of the string and the first placeholder (prefix)"] # [doc = " 2. Between two placeholders (infix)"] # [doc = " 3. Between the final placeholder and the end of the string (suffix)"] Literal (& 'a str) , }
};
}

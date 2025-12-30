// Generated macro for PatternItemCow (enum)
macro_rules! Depcrate_commonPatternItemCow {
() => {
// Module: crate::common
// Provides: {"PatternItemCow"}
// Dependencies: {}
# [doc = " A borrowed-or-owned item in a [`Pattern`]. Items are either string literals or placeholders."] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [doc = ""] # [doc = " [`Pattern`]: crate::Pattern"] # [derive (Debug , Clone , PartialEq , Eq , PartialOrd , Ord)] # [allow (clippy :: exhaustive_enums)] # [cfg (feature = "alloc")] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize , serde :: Serialize))] pub enum PatternItemCow < 'a , T > { # [doc = " A placeholder of the type specified on this [`PatternItemCow`]."] Placeholder (T) , # [doc = " A string literal. This can occur in one of three places:"] # [doc = ""] # [doc = " 1. Between the start of the string and the first placeholder (prefix)"] # [doc = " 2. Between two placeholders (infix)"] # [doc = " 3. Between the final placeholder and the end of the string (suffix)"] # [cfg_attr (feature = "serde" , serde (borrow))] Literal (Cow < 'a , str >) , }
};
}

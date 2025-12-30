// Generated macro for PatternItem (enum)
macro_rules! Depcrate_provider_pattern_itemPatternItem {
() => {
// Module: crate::provider::pattern::item
// Provides: {"PatternItem"}
// Dependencies: {}
# [doc = " An element of a [`Pattern`](super::runtime::Pattern)."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Debug , PartialEq , Eq , Clone , Copy)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_datetime :: provider :: pattern :: item))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [allow (clippy :: exhaustive_enums)] pub enum PatternItem { # [doc = " A field, like \"abbreviated months\". Mostly follows UTS 35."] Field (Field) , # [doc = " A literal code point."] Literal (char) , }
};
}

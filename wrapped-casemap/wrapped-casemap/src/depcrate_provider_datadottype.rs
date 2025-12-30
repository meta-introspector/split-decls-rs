// Generated macro for DotType (enum)
macro_rules! Depcrate_provider_dataDotType {
() => {
// Module: crate::provider::data
// Provides: {"DotType"}
// Dependencies: {}
# [doc = " The dot type of a Unicode character. This indicates how dotted"] # [doc = " letters (like `i` and `j`) combine with accents placed above the"] # [doc = " letter."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_casemap :: provider :: data))] # [derive (Default)] pub enum DotType { # [doc = " Normal characters with combining class 0"] # [default] NoDot = 0 , # [doc = " Soft-dotted characters with combining class 0"] SoftDotted = 1 , # [doc = " \"Above\" accents with combining class 230"] Above = 2 , # [doc = " Other accent characters"] OtherAccent = 3 , }
};
}

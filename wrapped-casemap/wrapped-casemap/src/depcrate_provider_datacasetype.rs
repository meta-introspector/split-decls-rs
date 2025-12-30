// Generated macro for CaseType (enum)
macro_rules! Depcrate_provider_dataCaseType {
() => {
// Module: crate::provider::data
// Provides: {"CaseType"}
// Dependencies: {}
# [doc = " The case of a Unicode character"] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_casemap :: provider :: data))] pub enum CaseType { # [doc = " Lowercase letter"] Lower = 1 , # [doc = " Uppercase letter"] Upper = 2 , # [doc = " Titlecase letter"] Title = 3 , }
};
}

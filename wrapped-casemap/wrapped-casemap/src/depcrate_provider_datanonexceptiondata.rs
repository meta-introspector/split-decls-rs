// Generated macro for NonExceptionData (struct)
macro_rules! Depcrate_provider_dataNonExceptionData {
() => {
// Module: crate::provider::data
// Provides: {"NonExceptionData"}
// Dependencies: {}
# [doc = " Data that is stored in CaseMapData when it is *not* an exception"] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_casemap :: provider :: data))] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct NonExceptionData { # [doc = " Whether or not the type is case-sensitive"] pub sensitive : bool , # [doc = " The \"dot type\""] pub dot_type : DotType , }
};
}

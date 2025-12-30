// Generated macro for CaseMapData (struct)
macro_rules! Depcrate_provider_dataCaseMapData {
() => {
// Module: crate::provider::data
// Provides: {"CaseMapData"}
// Dependencies: {}
# [doc = " Case mapping data associated with a single code point"] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_casemap :: provider :: data))] pub struct CaseMapData { # [doc = " Whether this is default-ignoreable"] pub ignoreable : bool , # [doc = " The rest of the case mapping data"] pub kind : CaseMapDataKind , }
};
}

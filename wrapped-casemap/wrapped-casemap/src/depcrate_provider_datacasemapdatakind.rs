// Generated macro for CaseMapDataKind (enum)
macro_rules! Depcrate_provider_dataCaseMapDataKind {
() => {
// Module: crate::provider::data
// Provides: {"CaseMapDataKind"}
// Dependencies: {}
# [doc = " A subset of case mapping data associated with a single code point"] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_casemap :: provider :: data))] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub enum CaseMapDataKind { # [doc = " This code point is an exception. Provides the case type of its own case"] # [doc = " and the exception index stored in [`CaseMapExceptions`]"] # [doc = ""] # [doc = " [`CaseMapExceptions`]: crate::provider::exceptions::CaseMapExceptions"] Exception (Option < CaseType > , u16) , # [doc = " This code point is uncased, and has the following extra data"] Uncased (NonExceptionData) , # [doc = " This code point is cased. We store the extra data, its case type, and a *delta*"] # [doc = " that can be used to get its casemapped codepoint."] Delta (NonExceptionData , CaseType , i16) , }
};
}

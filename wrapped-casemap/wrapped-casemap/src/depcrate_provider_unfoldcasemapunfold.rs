// Generated macro for CaseMapUnfold (struct)
macro_rules! Depcrate_provider_unfoldCaseMapUnfold {
() => {
// Module: crate::provider::unfold
// Provides: {"CaseMapUnfold"}
// Dependencies: {}
# [doc = " Reverse case folding data. Maps from multi-character strings back"] # [doc = " to code-points that fold to those strings."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_casemap :: provider))] # [derive (Debug , PartialEq , Clone , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [yoke (prove_covariance_manually)] pub struct CaseMapUnfold < 'data > { # [cfg_attr (feature = "serde" , serde (borrow))] # [doc = " The actual map. Maps from strings to a list of codepoints, stored as a contiguous UTF-8 string"] pub map : ZeroMap < 'data , PotentialUtf8 , str > , }
};
}

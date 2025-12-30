// Generated macro for SpecialCasePattern (struct)
macro_rules! Depcrate_providerSpecialCasePattern {
() => {
// Module: crate::provider
// Provides: {"SpecialCasePattern"}
// Dependencies: {}
# [doc = " The special case of a [`ConditionalListJoinerPattern`]"] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Clone , Debug , PartialEq , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_list :: provider))] pub struct SpecialCasePattern < 'data > { # [doc = " The condition on the following element"] pub condition : SerdeDFA < 'data > , # [doc = " The pattern if the condition matches"] pub pattern : ListJoinerPattern < 'data > , }
};
}

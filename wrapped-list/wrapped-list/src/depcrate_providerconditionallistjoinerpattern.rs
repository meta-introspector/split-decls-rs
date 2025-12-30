// Generated macro for ConditionalListJoinerPattern (struct)
macro_rules! Depcrate_providerConditionalListJoinerPattern {
() => {
// Module: crate::provider
// Provides: {"ConditionalListJoinerPattern"}
// Dependencies: {}
# [doc = " A pattern that can behave conditionally on the next element."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Clone , Debug , PartialEq , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_list :: provider))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] pub struct ConditionalListJoinerPattern < 'data > { # [doc = " The default pattern"] # [cfg_attr (feature = "serde" , serde (borrow))] pub default : ListJoinerPattern < 'data > , # [doc = " And optional special case"] # [cfg_attr (feature = "serde" , serde (borrow , deserialize_with = "SpecialCasePattern::deserialize_option"))] pub special_case : Option < SpecialCasePattern < 'data > > , }
};
}

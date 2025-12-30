// Generated macro for ListJoinerPattern (struct)
macro_rules! Depcrate_providerListJoinerPattern {
() => {
// Module: crate::provider
// Provides: {"ListJoinerPattern"}
// Dependencies: {}
# [doc = " A pattern containing two numeric placeholders (\"{0}, and {1}.\")"] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Clone , Debug , PartialEq , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize))] pub struct ListJoinerPattern < 'data > { # [doc = " The pattern string without the placeholders"] pub (crate) string : VarZeroCow < 'data , str > , # [doc = " The index of the first placeholder. Always <= index_1."] # [cfg_attr (feature = "datagen" , serde (skip))] pub (crate) index_0 : u8 , # [doc = " The index of the second placeholder. Always < string.len()."] pub (crate) index_1 : u8 , }
};
}

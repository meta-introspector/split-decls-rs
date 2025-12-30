// Generated macro for ListFormatterPatterns (struct)
macro_rules! Depcrate_providerListFormatterPatterns {
() => {
// Module: crate::provider
// Provides: {"ListFormatterPatterns"}
// Dependencies: {}
# [doc = " Symbols and metadata required for [`ListFormatter`](crate::ListFormatter)."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Clone , Debug , PartialEq , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_list :: provider))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] pub struct ListFormatterPatterns < 'data > { # [doc = " The start pattern"] # [cfg_attr (feature = "serde" , serde (borrow))] pub start : ListJoinerPattern < 'data > , # [doc = " The middle pattern. It doesn't need to be a pattern because it has to start with `{0}`"] # [doc = " and end with `{1}`, so we just store the string in between."] # [cfg_attr (feature = "serde" , serde (borrow))] pub middle : VarZeroCow < 'data , str > , # [doc = " The end pattern"] # [cfg_attr (feature = "serde" , serde (borrow))] pub end : ConditionalListJoinerPattern < 'data > , # [doc = " The pair pattern, if it's different from the end pattern."] # [cfg_attr (feature = "serde" , serde (borrow))] pub pair : Option < ConditionalListJoinerPattern < 'data > > , }
};
}

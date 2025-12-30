// Generated macro for GroupingSizes (struct)
macro_rules! Depcrate_providerGroupingSizes {
() => {
// Module: crate::provider
// Provides: {"GroupingSizes"}
// Dependencies: {}
# [doc = " A collection of settings expressing where to put grouping separators in a decimal number."] # [doc = " For example, `1,000,000` has two grouping separators, positioned along every 3 digits."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Debug , PartialEq , Clone , yoke :: Yokeable , Copy , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_decimal :: provider))] pub struct GroupingSizes { # [doc = " The size of the first (lowest-magnitude) group."] # [doc = ""] # [doc = " If 0, grouping separators will never be shown."] pub primary : u8 , # [doc = " The size of groups after the first group."] # [doc = ""] # [doc = " If 0, defaults to be the same as `primary`."] pub secondary : u8 , # [doc = " The minimum number of digits required before the first group. For example, if `primary=3`"] # [doc = " and `min_grouping=2`, grouping separators will be present on 10,000 and above."] pub min_grouping : u8 , }
};
}

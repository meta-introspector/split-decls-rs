// Generated macro for Month (enum)
macro_rules! Depcrate_provider_fields_componentsMonth {
() => {
// Module: crate::provider::fields::components
// Provides: {"Month"}
// Dependencies: {}
# [doc = " Options for displaying a Month for the `components::`[`Bag`]."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. It can be enabled with the `experimental` Cargo feature"] # [doc = " of the icu meta-crate. Use with caution."] # [doc = " <a href=\"https://github.com/unicode-org/icu4x/issues/1317\">#1317</a>"] # [doc = " </div>"] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize) , serde (rename_all = "kebab-case"))] # [non_exhaustive] pub enum Month { # [doc = " The numeric value of the month, such as \"4\"."] Numeric , # [doc = " The two-digit value of the month, such as \"04\"."] TwoDigit , # [doc = " The long value of the month, such as \"April\"."] Long , # [doc = " The short value of the month, such as \"Apr\"."] Short , # [doc = " The narrow value of the month, such as \"A\"."] Narrow , }
};
}

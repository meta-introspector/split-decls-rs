// Generated macro for Numeric (enum)
macro_rules! Depcrate_provider_fields_componentsNumeric {
() => {
// Module: crate::provider::fields::components
// Provides: {"Numeric"}
// Dependencies: {}
# [doc = " A numeric component for the `components::`[`Bag`]. It is used for the year, day, hour, minute,"] # [doc = " and second."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. It can be enabled with the `experimental` Cargo feature"] # [doc = " of the icu meta-crate. Use with caution."] # [doc = " <a href=\"https://github.com/unicode-org/icu4x/issues/1317\">#1317</a>"] # [doc = " </div>"] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize) , serde (rename_all = "kebab-case"))] # [non_exhaustive] pub enum Numeric { # [doc = " Display the numeric value. For instance in a year this would be \"1970\"."] Numeric , # [doc = " Display the two digit value. For instance in a year this would be \"70\"."] TwoDigit , }
};
}

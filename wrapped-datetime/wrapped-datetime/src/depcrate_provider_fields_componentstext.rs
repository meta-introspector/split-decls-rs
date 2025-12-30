// Generated macro for Text (enum)
macro_rules! Depcrate_provider_fields_componentsText {
() => {
// Module: crate::provider::fields::components
// Provides: {"Text"}
// Dependencies: {}
# [doc = " A text component for the `components::`[`Bag`]. It is used for the era and weekday."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. It can be enabled with the `experimental` Cargo feature"] # [doc = " of the icu meta-crate. Use with caution."] # [doc = " <a href=\"https://github.com/unicode-org/icu4x/issues/1317\">#1317</a>"] # [doc = " </div>"] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize) , serde (rename_all = "kebab-case"))] # [non_exhaustive] pub enum Text { # [doc = " Display the long form of the text, such as \"Wednesday\" for the weekday."] # [doc = " In UTS-35, known as \"Wide\" (4 letters)"] Long , # [doc = " Display the short form of the text, such as \"Wed\" for the weekday."] # [doc = " In UTS-35, known as \"Abbreviated\" (3 letters)"] Short , # [doc = " Display the narrow form of the text, such as \"W\" for the weekday."] # [doc = " In UTS-35, known as \"Narrow\" (5 letters)"] Narrow , }
};
}

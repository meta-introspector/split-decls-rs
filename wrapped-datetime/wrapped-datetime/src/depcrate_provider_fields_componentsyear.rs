// Generated macro for Year (enum)
macro_rules! Depcrate_provider_fields_componentsYear {
() => {
// Module: crate::provider::fields::components
// Provides: {"Year"}
// Dependencies: {}
# [doc = " Options for displaying a Year for the `components::`[`Bag`]."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. It can be enabled with the `experimental` Cargo feature"] # [doc = " of the icu meta-crate. Use with caution."] # [doc = " <a href=\"https://github.com/unicode-org/icu4x/issues/1317\">#1317</a>"] # [doc = " </div>"] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize) , serde (rename_all = "kebab-case"))] # [non_exhaustive] pub enum Year { # [doc = " The numeric value of the year, such as \"2018\" for 2018-12-31."] Numeric , # [doc = " The two-digit value of the year, such as \"18\" for 2018-12-31."] TwoDigit , # [doc = " The numeric value of the year in \"week-of-year\", such as \"2019\" in"] # [doc = " \"week 01 of 2019\" for the week of 2018-12-31 according to the ISO calendar."] NumericWeekOf , # [doc = " The numeric value of the year in \"week-of-year\", such as \"19\" in"] # [doc = " \"week 01 '19\" for the week of 2018-12-31 according to the ISO calendar."] TwoDigitWeekOf , }
};
}

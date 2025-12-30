// Generated macro for Week (enum)
macro_rules! Depcrate_provider_fields_componentsWeek {
() => {
// Module: crate::provider::fields::components
// Provides: {"Week"}
// Dependencies: {}
# [doc = " Options for displaying the current week number for the `components::`[`Bag`]."] # [doc = ""] # [doc = " Week numbers are relative to either a month or year, e.g. 'week 3 of January' or 'week 40 of 2000'."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. It can be enabled with the `experimental` Cargo feature"] # [doc = " of the icu meta-crate. Use with caution."] # [doc = " <a href=\"https://github.com/unicode-org/icu4x/issues/1317\">#1317</a>"] # [doc = " </div>"] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize) , serde (rename_all = "kebab-case"))] # [non_exhaustive] pub enum Week { # [doc = " The week of the month, such as the \"3\" in \"week 3 of January\"."] WeekOfMonth , # [doc = " The numeric value of the week of the year, such as the \"8\" in \"week 8 of 2000\"."] NumericWeekOfYear , # [doc = " The two-digit value of the week of the year, such as the \"08\" in \"2000-W08\"."] TwoDigitWeekOfYear , }
};
}

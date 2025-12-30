// Generated macro for Day (enum)
macro_rules! Depcrate_provider_fields_componentsDay {
() => {
// Module: crate::provider::fields::components
// Provides: {"Day"}
// Dependencies: {}
# [doc = " Options for displaying the current day of the month or year."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. It can be enabled with the `experimental` Cargo feature"] # [doc = " of the icu meta-crate. Use with caution."] # [doc = " <a href=\"https://github.com/unicode-org/icu4x/issues/1317\">#1317</a>"] # [doc = " </div>"] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize) , serde (rename_all = "kebab-case"))] # [non_exhaustive] pub enum Day { # [doc = " The numeric value of the day of month, such as the \"2\" in July 2 1984."] NumericDayOfMonth , # [doc = " The two digit value of the day of month, such as the \"02\" in 1984-07-02."] TwoDigitDayOfMonth , # [doc = " The day of week in this month, such as the \"2\" in 2nd Wednesday of July."] DayOfWeekInMonth , # [doc = " The day of year (numeric)."] DayOfYear , # [doc = " The modified Julian day (numeric)"] ModifiedJulianDay , }
};
}

// Generated macro for Saving (enum)
macro_rules! Depcrate_tableSaving {
() => {
// Module: crate::table
// Provides: {"Saving"}
// Dependencies: {}
# [doc = " The amount of daylight saving time (DST) to apply to this timespan. This"] # [doc = " is a special type for a certain field in a zone line, which can hold"] # [doc = " different types of value."] # [doc = ""] # [doc = " This is the owned version of the `Saving` type in the `line` module."] # [derive (PartialEq , Debug)] pub enum Saving { # [doc = " Just stick to the base offset."] NoSaving , # [doc = " This amount of time should be saved while this timespan is in effect."] # [doc = " (This is the equivalent to there being a single one-off rule with the"] # [doc = " given amount of time to save)."] OneOff (i64) , # [doc = " All rules with the given name should apply while this timespan is in"] # [doc = " effect."] Multiple (String) , }
};
}

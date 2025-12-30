// Generated macro for RuleInfo (struct)
macro_rules! Depcrate_tableRuleInfo {
() => {
// Module: crate::table
// Provides: {"RuleInfo"}
// Dependencies: {}
# [doc = " An owned rule definition line."] # [doc = ""] # [doc = " This mimics the `Rule` struct in the `line` module, only its uses owned"] # [doc = " Strings instead of string slices, and has had some pre-processing"] # [doc = " applied to it."] # [derive (PartialEq , Debug)] pub struct RuleInfo { # [doc = " The year that this rule *starts* applying."] pub from_year : Year , # [doc = " The year that this rule *finishes* applying, inclusive, or `None` if"] # [doc = " it applies up until the end of this timespan."] pub to_year : Option < Year > , # [doc = " The month it applies on."] pub month : Month , # [doc = " The day it applies on."] pub day : DaySpec , # [doc = " The exact time it applies on."] pub time : i64 , # [doc = " The type of time that time is."] pub time_type : TimeType , # [doc = " The amount of time to save."] pub time_to_add : i64 , # [doc = " Any extra letters that should be added to this time zone’s"] # [doc = " abbreviation, in place of `%s`."] pub letters : Option < String > , }
};
}

// Generated macro for DateFields (enum)
macro_rules! Depcrate_fieldsets_builderDateFields {
() => {
// Module: crate::fieldsets::builder
// Provides: {"DateFields"}
// Dependencies: {}
# [doc = " An enumeration over all possible date and calendar period field sets"] # [doc = " without options."] # [doc = ""] # [doc = " This is a builder enum. See [`builder`](crate::fieldsets::builder)."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] # [cfg_attr (all (feature = "serde" , feature = "experimental") , derive (serde :: Serialize , serde :: Deserialize))] # [non_exhaustive] pub enum DateFields { # [doc = " The day of the month, as in"] # [doc = " “on the 1st”."] D , # [doc = " The month and day of the month, as in"] # [doc = " “January 1st”."] MD , # [doc = " The year, month, and day of the month, as in"] # [doc = " “January 1st, 2000”."] YMD , # [doc = " The day of the month and day of the week, as in"] # [doc = " “Saturday 1st”."] DE , # [doc = " The month, day of the month, and day of the week, as in"] # [doc = " “Saturday, January 1st”."] MDE , # [doc = " The year, month, day of the month, and day of the week, as in"] # [doc = " “Saturday, January 1st, 2000”."] YMDE , # [doc = " The day of the week alone, as in"] # [doc = " “Saturday”."] E , # [doc = " A standalone month, as in"] # [doc = " “January”."] M , # [doc = " A month and year, as in"] # [doc = " “January 2000”."] YM , # [doc = " A year, as in"] # [doc = " “2000”."] Y , }
};
}

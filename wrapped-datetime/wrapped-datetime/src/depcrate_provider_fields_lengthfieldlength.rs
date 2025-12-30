// Generated macro for FieldLength (enum)
macro_rules! Depcrate_provider_fields_lengthFieldLength {
() => {
// Module: crate::provider::fields::length
// Provides: {"FieldLength"}
// Dependencies: {}
# [doc = " An enum representing the length of a field within a date or time formatting pattern string."] # [doc = ""] # [doc = " Such strings represent fields as a letter occurring 1 or more times in a row, ex:"] # [doc = " `MMM`, `dd`, `y`.  See the"] # [doc = " [LDML documentation in UTS 35](https://unicode.org/reports/tr35/tr35-dates.html#Date_Format_Patterns)"] # [doc = " for more details."] # [derive (Debug , Eq , PartialEq , Clone , Copy , Ord , PartialOrd)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_datetime :: fields))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [allow (clippy :: exhaustive_enums)] pub enum FieldLength { # [doc = " Numeric: minimum digits"] # [doc = ""] # [doc = " Text: same as [`Self::Three`]"] One , # [doc = " Numeric: pad to 2 digits"] # [doc = ""] # [doc = " Text: same as [`Self::Three`]"] Two , # [doc = " Numeric: pad to 3 digits"] # [doc = ""] # [doc = " Text: Abbreviated format."] Three , # [doc = " Numeric: pad to 4 digits"] # [doc = ""] # [doc = " Text: Wide format."] Four , # [doc = " Numeric: pad to 5 digits"] # [doc = ""] # [doc = " Text: Narrow format."] Five , # [doc = " Numeric: pad to 6 digits"] # [doc = ""] # [doc = " Text: Short format."] Six , # [doc = " FieldLength::One (numeric), but overridden with a different numbering system"] NumericOverride (FieldNumericOverrides) , }
};
}

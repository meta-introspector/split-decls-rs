// Generated macro for Weekday (enum)
macro_rules! Depcrate_typesWeekday {
() => {
// Module: crate::types
// Provides: {"Weekday"}
// Dependencies: {}
# [doc = " A weekday in a 7-day week, according to ISO-8601."] # [doc = ""] # [doc = " The discriminant values correspond to ISO-8601 weekday numbers (Monday = 1, Sunday = 7)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::calendar::types::Weekday;"] # [doc = ""] # [doc = " assert_eq!(1, Weekday::Monday as usize);"] # [doc = " assert_eq!(7, Weekday::Sunday as usize);"] # [doc = " ```"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [allow (missing_docs)] # [repr (i8)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_calendar :: types))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [allow (clippy :: exhaustive_enums)] pub enum Weekday { Monday = 1 , Tuesday , Wednesday , Thursday , Friday , Saturday , Sunday , }
};
}

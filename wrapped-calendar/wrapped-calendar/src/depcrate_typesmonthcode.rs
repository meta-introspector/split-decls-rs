// Generated macro for MonthCode (struct)
macro_rules! Depcrate_typesMonthCode {
() => {
// Module: crate::types
// Provides: {"MonthCode"}
// Dependencies: {}
# [doc = " String representation of a [`Month`]"] # [derive (Clone , Copy , Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] # [allow (clippy :: exhaustive_structs)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_calendar :: types))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] pub struct MonthCode (pub TinyAsciiStr < 4 >) ;
};
}

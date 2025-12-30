// Generated macro for ZoneStyle (enum)
macro_rules! Depcrate_fieldsets_builderZoneStyle {
() => {
// Module: crate::fieldsets::builder
// Provides: {"ZoneStyle"}
// Dependencies: {}
# [doc = " An enumeration over all possible time zone styles."] # [doc = ""] # [doc = " This is a builder enum. See [`builder`](crate::fieldsets::builder)."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] # [cfg_attr (all (feature = "serde" , feature = "experimental") , derive (serde :: Serialize , serde :: Deserialize))] # [non_exhaustive] pub enum ZoneStyle { # [doc = " The long specific non-location format, as in"] # [doc = " “Pacific Daylight Time”."] SpecificLong , # [doc = " The short specific non-location format, as in"] # [doc = " “PDT”."] SpecificShort , # [doc = " The long offset format, as in"] # [doc = " “GMT−8:00”."] LocalizedOffsetLong , # [doc = " The short offset format, as in"] # [doc = " “GMT−8”."] LocalizedOffsetShort , # [doc = " The long generic non-location format, as in"] # [doc = " “Pacific Time”."] GenericLong , # [doc = " The short generic non-location format, as in"] # [doc = " “PT”."] GenericShort , # [doc = " The location format, as in"] # [doc = " “Los Angeles time”."] Location , # [doc = " The exemplar city format, as in"] # [doc = " “Los Angeles”."] ExemplarCity , }
};
}

// Generated macro for Configuration (struct)
macro_rules! DepcrateConfiguration {
() => {
// Module: crate
// Provides: {"Configuration"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Debug)] pub struct Configuration { pub time_offset : Option < f64 > , pub original_uris : Option < Vec < String > > , }
};
}

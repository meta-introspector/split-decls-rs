// Generated macro for Trace (struct)
macro_rules! DepcrateTrace {
() => {
// Module: crate
// Provides: {"Trace"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Debug)] pub struct Trace { pub vantage_point : VantagePoint , pub title : Option < String > , pub description : Option < String > , pub configuration : Option < Configuration > , pub common_fields : Option < CommonFields > , pub events : Vec < Event > , }
};
}

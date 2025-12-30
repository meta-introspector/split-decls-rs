// Generated macro for VantagePoint (struct)
macro_rules! DepcrateVantagePoint {
() => {
// Module: crate
// Provides: {"VantagePoint"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] pub struct VantagePoint { pub name : Option < String > , # [serde (rename = "type")] pub ty : VantagePointType , pub flow : Option < VantagePointType > , }
};
}

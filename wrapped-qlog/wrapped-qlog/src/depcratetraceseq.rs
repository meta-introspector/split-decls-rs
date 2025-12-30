// Generated macro for TraceSeq (struct)
macro_rules! DepcrateTraceSeq {
() => {
// Module: crate
// Provides: {"TraceSeq"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Debug)] pub struct TraceSeq { pub vantage_point : VantagePoint , pub title : Option < String > , pub description : Option < String > , pub configuration : Option < Configuration > , pub common_fields : Option < CommonFields > , }
};
}

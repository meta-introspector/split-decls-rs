// Generated macro for QlogSeq (struct)
macro_rules! DepcrateQlogSeq {
() => {
// Module: crate
// Provides: {"QlogSeq"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , Debug)] pub struct QlogSeq { pub qlog_version : String , pub qlog_format : String , pub title : Option < String > , pub description : Option < String > , pub summary : Option < String > , pub trace : TraceSeq , }
};
}

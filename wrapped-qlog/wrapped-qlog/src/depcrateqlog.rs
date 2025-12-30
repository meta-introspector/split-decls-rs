// Generated macro for Qlog (struct)
macro_rules! DepcrateQlog {
() => {
// Module: crate
// Provides: {"Qlog"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone)] pub struct Qlog { pub qlog_version : String , pub qlog_format : String , pub title : Option < String > , pub description : Option < String > , pub summary : Option < String > , pub traces : Vec < Trace > , }
};
}

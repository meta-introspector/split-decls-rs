// Generated macro for v1 (module)
macro_rules! Depcratev1 {
() => {
// Module: crate
// Provides: {"v1"}
// Dependencies: {}
pub mod v1 { use super :: * ; pub const SCHEMA_VERSION : SchemaVersion = SchemaVersion { schema_version : 1 } ; # [derive (Clone , Debug , Deserialize , Serialize)] pub struct JsonFrame { pub data : String , # [doc = " Unix timestamp in nanoseconds"] pub host_timestamp : i64 , pub level : Option < Level > , pub location : Location , pub target_timestamp : String , } # [derive (Clone , Debug , Deserialize , Serialize)] pub struct Location { pub file : Option < String > , pub line : Option < u32 > , pub module_path : Option < ModulePath > , } # [derive (Clone , Debug , Deserialize , Serialize)] pub struct ModulePath { pub crate_name : String , pub modules : Vec < String > , pub function : String , } }
};
}

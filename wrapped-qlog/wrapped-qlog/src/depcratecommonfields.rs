// Generated macro for CommonFields (struct)
macro_rules! DepcrateCommonFields {
() => {
// Module: crate
// Provides: {"CommonFields"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , Default , PartialEq , Debug)] pub struct CommonFields { pub group_id : Option < String > , pub protocol_type : Option < Vec < String > > , pub reference_time : Option < f64 > , pub time_format : Option < String > , }
};
}

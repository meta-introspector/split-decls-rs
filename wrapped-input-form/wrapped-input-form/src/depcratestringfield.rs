// Generated macro for StringField (struct)
macro_rules! DepcrateStringField {
() => {
// Module: crate
// Provides: {"StringField"}
// Dependencies: {}
# [doc = " A new-type representing a string field with a label."] # [derive (Debug , Serialize)] struct StringField { # [serde (skip)] label : & 'static str , value : String , }
};
}

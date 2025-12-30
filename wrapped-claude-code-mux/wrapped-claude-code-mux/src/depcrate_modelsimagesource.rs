// Generated macro for ImageSource (struct)
macro_rules! Depcrate_modelsImageSource {
() => {
// Module: crate::models
// Provides: {"ImageSource"}
// Dependencies: {}
# [doc = " Image source for vision API"] # [derive (Debug , Clone , Deserialize , Serialize)] pub struct ImageSource { pub r#type : String , # [serde (skip_serializing_if = "Option::is_none")] pub media_type : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub data : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub url : Option < String > , }
};
}

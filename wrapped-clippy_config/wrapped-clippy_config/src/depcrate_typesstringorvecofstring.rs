// Generated macro for StringOrVecOfString (enum)
macro_rules! Depcrate_typesStringOrVecOfString {
() => {
// Module: crate::types
// Provides: {"StringOrVecOfString"}
// Dependencies: {}
# [doc = " Helper struct for deserializing the [`SourceItemOrderingWithinModuleItemGroupings`]."] # [derive (Deserialize)] # [serde (untagged)] enum StringOrVecOfString { String (String) , Vec (Vec < String >) , }
};
}

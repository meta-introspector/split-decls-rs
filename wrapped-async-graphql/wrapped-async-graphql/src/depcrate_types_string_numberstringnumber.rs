// Generated macro for StringNumber (struct)
macro_rules! Depcrate_types_string_numberStringNumber {
() => {
// Module: crate::types::string_number
// Provides: {"StringNumber"}
// Dependencies: {}
# [doc = " A numeric value represented by a string."] # [derive (Clone , Ord , PartialOrd , Eq , PartialEq , Hash , Debug , Serialize , Deserialize)] # [serde (transparent)] # [cfg_attr (docsrs , doc (cfg (feature = "string_number")))] pub struct StringNumber < T : Num + Display > (pub T) ;
};
}

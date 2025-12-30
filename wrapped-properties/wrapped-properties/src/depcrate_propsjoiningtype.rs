// Generated macro for JoiningType (struct)
macro_rules! Depcrate_propsJoiningType {
() => {
// Module: crate::props
// Provides: {"JoiningType"}
// Dependencies: {}
# [doc = " Enumerated property Joining_Type."] # [doc = ""] # [doc = " See Section 9.2, Arabic Cursive Joining in The Unicode Standard for the summary of"] # [doc = " each property value."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::{props::JoiningType, CodePointMapData};"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     CodePointMapData::<JoiningType>::new().get('ؠ'),"] # [doc = "     JoiningType::DualJoining"] # [doc = " ); // U+0620: Arabic Letter Kashmiri Yeh"] # [doc = " assert_eq!("] # [doc = "     CodePointMapData::<JoiningType>::new().get('𐫍'),"] # [doc = "     JoiningType::LeftJoining"] # [doc = " ); // U+10ACD: Manichaean Letter Heth"] # [doc = " ```"] # [derive (Copy , Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [allow (clippy :: exhaustive_structs)] # [repr (transparent)] pub struct JoiningType (pub (crate) u8) ;
};
}

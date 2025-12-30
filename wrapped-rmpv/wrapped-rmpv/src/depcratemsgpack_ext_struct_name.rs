// Generated macro for MSGPACK_EXT_STRUCT_NAME (const)
macro_rules! DepcrateMSGPACK_EXT_STRUCT_NAME {
() => {
// Module: crate
// Provides: {"MSGPACK_EXT_STRUCT_NAME"}
// Dependencies: {}
# [doc = " Name of Serde newtype struct to Represent Msgpack's Ext"] # [doc = " Msgpack Ext: Ext(tag, binary)"] # [doc = " Serde data model: _ExtStruct((tag, binary))"] # [doc = " Example Serde impl for custom type:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " #[derive(Debug, PartialEq, Serialize, Deserialize)]"] # [doc = " #[serde(rename = \"_ExtStruct\")]"] # [doc = " struct ExtStruct((i8, serde_bytes::ByteBuf));"] # [doc = ""] # [doc = " test_round(ExtStruct((2, serde_bytes::ByteBuf::from(vec![5]))),"] # [doc = "            Value::Ext(2, vec![5]));"] # [doc = " ```"] pub const MSGPACK_EXT_STRUCT_NAME : & str = "_ExtStruct" ;
};
}

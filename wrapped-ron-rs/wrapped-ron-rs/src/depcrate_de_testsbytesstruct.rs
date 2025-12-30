// Generated macro for BytesStruct (struct)
macro_rules! Depcrate_de_testsBytesStruct {
() => {
// Module: crate::de::tests
// Provides: {"BytesStruct"}
// Dependencies: {}
# [derive (Debug , Deserialize , PartialEq)] struct BytesStruct { small : Vec < u8 > , # [serde (with = "serde_bytes")] large : Vec < u8 > , }
};
}

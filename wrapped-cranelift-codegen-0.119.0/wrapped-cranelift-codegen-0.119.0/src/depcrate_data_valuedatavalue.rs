// Generated macro for DataValue (enum)
macro_rules! Depcrate_data_valueDataValue {
() => {
// Module: crate::data_value
// Provides: {"DataValue"}
// Dependencies: {}
# [doc = " Represent a data value. Where [Value] is an SSA reference, [DataValue] is the type + value"] # [doc = " that would be referred to by a [Value]."] # [doc = ""] # [doc = " [Value]: crate::ir::Value"] # [allow (missing_docs)] # [derive (Clone , Debug , PartialOrd)] pub enum DataValue { I8 (i8) , I16 (i16) , I32 (i32) , I64 (i64) , I128 (i128) , F16 (Ieee16) , F32 (Ieee32) , F64 (Ieee64) , F128 (Ieee128) , V128 ([u8 ; 16]) , V64 ([u8 ; 8]) , }
};
}

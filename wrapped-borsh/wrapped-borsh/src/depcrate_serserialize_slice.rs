// Generated macro for serialize_slice (function)
macro_rules! Depcrate_serserialize_slice {
() => {
// Module: crate::ser
// Provides: {"serialize_slice"}
// Dependencies: {}
# [doc = " Helper method that is used to serialize a slice of data (without the length marker)."] # [inline] fn serialize_slice < T : BorshSerialize , W : Write > (data : & [T] , writer : & mut W) -> Result < () > { if let Some (u8_slice) = T :: u8_slice (data) { writer . write_all (u8_slice) ? ; } else { for item in data { item . serialize (writer) ? ; } } Ok (()) }
};
}

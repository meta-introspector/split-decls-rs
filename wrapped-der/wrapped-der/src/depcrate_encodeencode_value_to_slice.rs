// Generated macro for encode_value_to_slice (function)
macro_rules! Depcrate_encodeencode_value_to_slice {
() => {
// Module: crate::encode
// Provides: {"encode_value_to_slice"}
// Dependencies: {}
# [doc = " Encodes value only (without tag + length) to a slice."] pub (crate) fn encode_value_to_slice < 'a , T > (buf : & 'a mut [u8] , value : & T) -> Result < & 'a [u8] > where T : EncodeValue , { let mut writer = SliceWriter :: new (buf) ; value . encode_value (& mut writer) ? ; writer . finish () }
};
}

// Generated macro for strip_padding (function)
macro_rules! Depcrate_frame_utilstrip_padding {
() => {
// Module: crate::frame::util
// Provides: {"strip_padding"}
// Dependencies: {}
# [doc = " Strip padding from the given payload."] # [doc = ""] # [doc = " It is assumed that the frame had the padded flag set. This means that the"] # [doc = " first byte is the length of the padding with that many"] # [doc = " 0 bytes expected to follow the actual payload."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " A slice of the given payload where the actual one is found and the length"] # [doc = " of the padding."] # [doc = ""] # [doc = " If the padded payload is invalid (e.g. the length of the padding is equal"] # [doc = " to the total length), returns `None`."] pub fn strip_padding (payload : & mut Bytes) -> Result < u8 , Error > { let payload_len = payload . len () ; if payload_len == 0 { return Err (Error :: TooMuchPadding) ; } let pad_len = payload [0] as usize ; if pad_len >= payload_len { return Err (Error :: TooMuchPadding) ; } payload . advance (1) ; payload . truncate (payload_len - pad_len - 1) ; Ok (pad_len as u8) }
};
}

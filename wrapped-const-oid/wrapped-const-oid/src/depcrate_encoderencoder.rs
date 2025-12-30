// Generated macro for Encoder (struct)
macro_rules! Depcrate_encoderEncoder {
() => {
// Module: crate::encoder
// Provides: {"Encoder"}
// Dependencies: {}
# [doc = " BER/DER encoder."] # [derive (Debug)] pub (crate) struct Encoder < const MAX_SIZE : usize > { # [doc = " Current state."] state : State , # [doc = " Bytes of the OID being BER-encoded in-progress."] bytes : [u8 ; MAX_SIZE] , # [doc = " Current position within the byte buffer."] cursor : usize , }
};
}

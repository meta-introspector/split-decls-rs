// Generated macro for GeneralPurpose (struct)
macro_rules! Depcrate_engine_general_purposeGeneralPurpose {
() => {
// Module: crate::engine::general_purpose
// Provides: {"GeneralPurpose"}
// Dependencies: {}
# [doc = " A general-purpose base64 engine."] # [doc = ""] # [doc = " - It uses no vector CPU instructions, so it will work on any system."] # [doc = " - It is reasonably fast (~2-3GiB/s)."] # [doc = " - It is not constant-time, though, so it is vulnerable to timing side-channel attacks. For loading cryptographic keys, etc, it is suggested to use the forthcoming constant-time implementation."] # [derive (Debug , Clone)] pub struct GeneralPurpose { encode_table : [u8 ; 64] , decode_table : [u8 ; 256] , config : GeneralPurposeConfig , }
};
}

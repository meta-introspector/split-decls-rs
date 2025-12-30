// Generated macro for LzmaEncoder (struct)
macro_rules! Depcrate_enc_encoderLzmaEncoder {
() => {
// Module: crate::enc::encoder
// Provides: {"LzmaEncoder"}
// Dependencies: {}
pub (crate) struct LzmaEncoder { pub (crate) coder : LzmaCoder , pub (crate) lz : LzEncoder , pub (crate) literal_encoder : LiteralEncoder , pub (crate) match_len_encoder : LengthEncoder , pub (crate) rep_len_encoder : LengthEncoder , pub (crate) data : LzmaEncData , }
};
}

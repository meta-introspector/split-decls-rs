// Generated macro for PrkMode (enum)
macro_rules! Depcrate_hkdfPrkMode {
() => {
// Module: crate::hkdf
// Provides: {"PrkMode"}
// Dependencies: {}
# [derive (Clone)] enum PrkMode { Expand { key_bytes : [u8 ; MAX_HKDF_PRK_LEN] , key_len : usize , } , ExtractExpand { secret : Arc < ZeroizeBoxSlice < u8 > > , salt : [u8 ; MAX_HKDF_SALT_LEN] , salt_len : usize , } , }
};
}

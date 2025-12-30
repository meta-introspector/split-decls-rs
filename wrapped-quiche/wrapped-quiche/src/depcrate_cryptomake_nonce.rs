// Generated macro for make_nonce (function)
macro_rules! Depcrate_cryptomake_nonce {
() => {
// Module: crate::crypto
// Provides: {"make_nonce"}
// Dependencies: {}
fn make_nonce (iv : & [u8] , counter : u64) -> [u8 ; MAX_NONCE_LEN] { let mut nonce = [0 ; MAX_NONCE_LEN] ; nonce . copy_from_slice (iv) ; for (a , b) in nonce [4 ..] . iter_mut () . zip (counter . to_be_bytes () . iter ()) { * a ^= b ; } nonce }
};
}

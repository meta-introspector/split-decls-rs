// Generated macro for create_cid_and_reset_token (function)
macro_rules! Depcrate_test_utilscreate_cid_and_reset_token {
() => {
// Module: crate::test_utils
// Provides: {"create_cid_and_reset_token"}
// Dependencies: {}
pub fn create_cid_and_reset_token (cid_len : usize ,) -> (ConnectionId < 'static > , u128) { let mut cid = vec ! [0 ; cid_len] ; rand :: rand_bytes (& mut cid [..]) ; let cid = ConnectionId :: from_ref (& cid) . into_owned () ; let mut reset_token = [0 ; 16] ; rand :: rand_bytes (& mut reset_token) ; let reset_token = u128 :: from_be_bytes (reset_token) ; (cid , reset_token) }
};
}

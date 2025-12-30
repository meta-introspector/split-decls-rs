// Generated macro for pipe_with_exchanged_cids (function)
macro_rules! Depcrate_testspipe_with_exchanged_cids {
() => {
// Module: crate::tests
// Provides: {"pipe_with_exchanged_cids"}
// Dependencies: {}
fn pipe_with_exchanged_cids (config : & mut Config , client_scid_len : usize , server_scid_len : usize , additional_cids : usize ,) -> test_utils :: Pipe { let mut pipe = test_utils :: Pipe :: with_config_and_scid_lengths (config , client_scid_len , server_scid_len ,) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; let mut c_cids = Vec :: new () ; let mut c_reset_tokens = Vec :: new () ; let mut s_cids = Vec :: new () ; let mut s_reset_tokens = Vec :: new () ; for i in 0 .. additional_cids { if client_scid_len > 0 { let (c_cid , c_reset_token) = test_utils :: create_cid_and_reset_token (client_scid_len) ; c_cids . push (c_cid) ; c_reset_tokens . push (c_reset_token) ; assert_eq ! (pipe . client . new_scid (& c_cids [i] , c_reset_tokens [i] , true) , Ok (i as u64 + 1)) ; } if server_scid_len > 0 { let (s_cid , s_reset_token) = test_utils :: create_cid_and_reset_token (server_scid_len) ; s_cids . push (s_cid) ; s_reset_tokens . push (s_reset_token) ; assert_eq ! (pipe . server . new_scid (& s_cids [i] , s_reset_tokens [i] , true) , Ok (i as u64 + 1)) ; } } assert_eq ! (pipe . advance () , Ok (())) ; if client_scid_len > 0 { assert_eq ! (pipe . server . available_dcids () , additional_cids) ; } if server_scid_len > 0 { assert_eq ! (pipe . client . available_dcids () , additional_cids) ; } assert_eq ! (pipe . server . path_event_next () , None) ; assert_eq ! (pipe . client . path_event_next () , None) ; pipe }
};
}

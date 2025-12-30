// Generated macro for cid_retirement (function)
macro_rules! Depcrate_testscid_retirement {
() => {
// Module: crate::tests
// Provides: {"cid_retirement"}
// Dependencies: {}
# [test] fn cid_retirement () { let _guard = subscribe () ; let mut pair = Pair :: default () ; let (client_ch , server_ch) = pair . connect () ; pair . server_conn_mut (server_ch) . rotate_local_cid (1 , Instant :: now ()) ; pair . drive () ; assert ! (! pair . client_conn_mut (client_ch) . is_closed ()) ; assert ! (! pair . server_conn_mut (server_ch) . is_closed ()) ; assert_matches ! (pair . client_conn_mut (client_ch) . active_rem_cid_seq () , 1) ; use crate :: LOC_CID_COUNT ; use crate :: cid_queue :: CidQueue ; let mut active_cid_num = CidQueue :: LEN as u64 ; active_cid_num = active_cid_num . min (LOC_CID_COUNT) ; let next_retire_prior_to = active_cid_num + 1 ; pair . client_conn_mut (client_ch) . ping () ; pair . server_conn_mut (server_ch) . rotate_local_cid (next_retire_prior_to , Instant :: now ()) ; pair . drive () ; assert ! (! pair . client_conn_mut (client_ch) . is_closed ()) ; assert ! (! pair . server_conn_mut (server_ch) . is_closed ()) ; assert_eq ! (pair . client_conn_mut (client_ch) . active_rem_cid_seq () , next_retire_prior_to ,) ; }
};
}

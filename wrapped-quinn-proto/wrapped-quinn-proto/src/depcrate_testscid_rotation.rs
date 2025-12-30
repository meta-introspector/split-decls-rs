// Generated macro for cid_rotation (function)
macro_rules! Depcrate_testscid_rotation {
() => {
// Module: crate::tests
// Provides: {"cid_rotation"}
// Dependencies: {}
# [test] fn cid_rotation () { let _guard = subscribe () ; const CID_TIMEOUT : Duration = Duration :: from_secs (2) ; let cid_generator_factory : fn () -> Box < dyn ConnectionIdGenerator > = | | Box :: new (* RandomConnectionIdGenerator :: new (8) . set_lifetime (CID_TIMEOUT)) ; let server = Endpoint :: new (Arc :: new (EndpointConfig { connection_id_generator_factory : Arc :: new (cid_generator_factory) , .. EndpointConfig :: default () }) , Some (Arc :: new (server_config ())) , true , None ,) ; let client = Endpoint :: new (Arc :: new (EndpointConfig :: default ()) , None , true , None) ; let mut pair = Pair :: new_from_endpoint (client , server) ; let (_ , server_ch) = pair . connect () ; let mut round : u64 = 1 ; let mut stop = pair . time ; let end = pair . time + 5 * CID_TIMEOUT ; use crate :: LOC_CID_COUNT ; use crate :: cid_queue :: CidQueue ; let mut active_cid_num = CidQueue :: LEN as u64 + 1 ; active_cid_num = active_cid_num . min (LOC_CID_COUNT) ; let mut left_bound = 0 ; let mut right_bound = active_cid_num - 1 ; while pair . time < end { stop += CID_TIMEOUT ; while pair . time < stop { if ! pair . step () { if let Some (time) = min_opt (pair . client . next_wakeup () , pair . server . next_wakeup ()) { pair . time = time ; } } } info ! ("Checking active cid sequence range before {:?} seconds" , round * CID_TIMEOUT . as_secs ()) ; let _bound = (left_bound , right_bound) ; assert_matches ! (pair . server_conn_mut (server_ch) . active_local_cid_seq () , _bound) ; round += 1 ; left_bound += active_cid_num ; right_bound += active_cid_num ; pair . drive_server () ; } }
};
}

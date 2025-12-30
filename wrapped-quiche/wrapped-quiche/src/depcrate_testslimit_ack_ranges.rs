// Generated macro for limit_ack_ranges (function)
macro_rules! Depcrate_testslimit_ack_ranges {
() => {
// Module: crate::tests
// Provides: {"limit_ack_ranges"}
// Dependencies: {}
# [rstest] fn limit_ack_ranges (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; let epoch = packet :: Epoch :: Application ; assert_eq ! (pipe . server . pkt_num_spaces [epoch] . recv_pkt_need_ack . len () , 0) ; let frames = [frame :: Frame :: Ping { mtu_probe : None } , frame :: Frame :: Padding { len : 3 } ,] ; let pkt_type = Type :: Short ; let mut last_packet_sent = 0 ; for _ in 0 .. 512 { let recv_count = pipe . server . recv_count ; last_packet_sent = pipe . client . next_pkt_num ; pipe . send_pkt_to_server (pkt_type , & frames , & mut buf) . unwrap () ; assert_eq ! (pipe . server . recv_count , recv_count + 1) ; pipe . client . next_pkt_num += 1 ; } assert_eq ! (pipe . server . pkt_num_spaces [epoch] . recv_pkt_need_ack . len () , MAX_ACK_RANGES) ; assert_eq ! (pipe . server . pkt_num_spaces [epoch] . recv_pkt_need_ack . first () , Some (last_packet_sent - ((MAX_ACK_RANGES as u64) - 1) * 2)) ; assert_eq ! (pipe . server . pkt_num_spaces [epoch] . recv_pkt_need_ack . last () , Some (last_packet_sent)) ; }
};
}

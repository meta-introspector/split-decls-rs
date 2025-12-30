// Generated macro for early_retransmit (function)
macro_rules! Depcrate_testsearly_retransmit {
() => {
// Module: crate::tests
// Provides: {"early_retransmit"}
// Dependencies: {}
# [rstest] # [doc = " Tests that old data is retransmitted on PTO."] fn early_retransmit (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; assert_eq ! (pipe . client . stream_send (0 , b"a" , false) , Ok (1)) ; assert_eq ! (pipe . advance () , Ok (())) ; assert_eq ! (pipe . client . stream_send (4 , b"b" , false) , Ok (1)) ; assert ! (pipe . client . send (& mut buf) . is_ok ()) ; let timer = pipe . client . timeout () . unwrap () ; std :: thread :: sleep (timer + Duration :: from_millis (1)) ; pipe . client . on_timeout () ; let epoch = packet :: Epoch :: Application ; assert_eq ! (pipe . client . paths . get_active () . expect ("no active") . recovery . loss_probes (epoch) , 1 ,) ; let (len , _) = pipe . client . send (& mut buf) . unwrap () ; assert_eq ! (pipe . client . paths . get_active () . expect ("no active") . recovery . loss_probes (epoch) , 0 ,) ; let frames = test_utils :: decode_pkt (& mut pipe . server , & mut buf [.. len]) . unwrap () ; let mut iter = frames . iter () ; iter . next () ; assert_eq ! (iter . next () , Some (& frame :: Frame :: Stream { stream_id : 4 , data : < RangeBuf >:: from (b"b" , 0 , false) , })) ; assert_eq ! (pipe . client . stats () . retrans , 1) ; }
};
}

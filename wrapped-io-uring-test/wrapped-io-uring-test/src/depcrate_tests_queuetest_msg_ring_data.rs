// Generated macro for test_msg_ring_data (function)
macro_rules! Depcrate_tests_queuetest_msg_ring_data {
() => {
// Module: crate::tests::queue
// Provides: {"test_msg_ring_data"}
// Dependencies: {}
pub fn test_msg_ring_data < S : squeue :: EntryMarker , C : cqueue :: EntryMarker > (ring : & mut IoUring < S , C > , test : & Test ,) -> anyhow :: Result < () > { use std :: os :: unix :: io :: AsRawFd ; require ! (test ; test . probe . is_supported (opcode :: MsgRingData :: CODE) ;) ; println ! ("test msg_ring_data") ; let mut dest_ring = IoUring :: new (1) ? ; let fd = types :: Fd (dest_ring . as_raw_fd ()) ; let result = 82 ; let user_data = 85 ; unsafe { ring . submission () . push (& opcode :: MsgRingData :: new (fd , result , user_data , None) . build () . into () ,) . expect ("queue is full") ; } ring . submit_and_wait (1) ? ; let source_cqes : Vec < cqueue :: Entry > = ring . completion () . map (Into :: into) . collect () ; assert_eq ! (source_cqes . len () , 1) ; assert_eq ! (source_cqes [0] . user_data () , 0) ; assert_eq ! (source_cqes [0] . result () , 0) ; assert_eq ! (source_cqes [0] . flags () , 0) ; let dest_cqes : Vec < cqueue :: Entry > = dest_ring . completion () . map (Into :: into) . collect () ; assert_eq ! (dest_cqes . len () , 1) ; assert_eq ! (dest_cqes [0] . user_data () , user_data) ; assert_eq ! (dest_cqes [0] . result () , result) ; assert_eq ! (dest_cqes [0] . flags () , 0) ; Ok (()) }
};
}

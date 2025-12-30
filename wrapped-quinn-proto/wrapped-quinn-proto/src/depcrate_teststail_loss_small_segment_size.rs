// Generated macro for tail_loss_small_segment_size (function)
macro_rules! Depcrate_teststail_loss_small_segment_size {
() => {
// Module: crate::tests
// Provides: {"tail_loss_small_segment_size"}
// Dependencies: {}
# [test] fn tail_loss_small_segment_size () { let _guard = subscribe () ; let mut pair = Pair :: default () ; let (client_ch , server_ch) = pair . connect () ; let server_stats = pair . server_conn_mut (server_ch) . stats () ; assert_eq ! (server_stats . frame_rx . datagram , 0) ; const DGRAM_LEN : usize = 1000 ; const DGRAM_NUM : u64 = 5 ; info ! ("Sending an ack-eliciting datagram") ; pair . client_conn_mut (client_ch) . ping () ; pair . drive_client () ; assert ! (! pair . server . inbound . is_empty ()) ; pair . server . inbound . clear () ; info ! ("stepping forward to PTO") ; pair . step () ; let server_stats = pair . server_conn_mut (server_ch) . stats () ; assert_eq ! (server_stats . frame_rx . datagram , 0) ; info ! ("Sending datagram batch") ; for _ in 0 .. DGRAM_NUM { pair . client_datagrams (client_ch) . send (vec ! [0 ; DGRAM_LEN] . into () , false) . unwrap () ; } pair . drive () ; let server_stats = pair . server_conn_mut (server_ch) . stats () ; assert_eq ! (server_stats . frame_rx . datagram , DGRAM_NUM) ; }
};
}

// Generated macro for tail_loss_respect_max_datagrams (function)
macro_rules! Depcrate_teststail_loss_respect_max_datagrams {
() => {
// Module: crate::tests
// Provides: {"tail_loss_respect_max_datagrams"}
// Dependencies: {}
# [test] fn tail_loss_respect_max_datagrams () { let _guard = subscribe () ; let client_config = { let mut c_config = client_config () ; let mut t_config = TransportConfig :: default () ; t_config . enable_segmentation_offload (false) ; c_config . transport_config (t_config . into ()) ; c_config } ; let mut pair = Pair :: default () ; let (client_ch , _) = pair . connect_with (client_config) ; const DGRAM_LEN : usize = 1000 ; const DGRAM_NUM : u64 = 5 ; info ! ("Sending an ack-eliciting datagram") ; pair . client_conn_mut (client_ch) . ping () ; pair . drive_client () ; assert ! (! pair . server . inbound . is_empty ()) ; pair . server . inbound . clear () ; info ! ("stepping forward to PTO") ; pair . step () ; info ! ("Sending datagram batch") ; for _ in 0 .. DGRAM_NUM { pair . client_datagrams (client_ch) . send (vec ! [0 ; DGRAM_LEN] . into () , false) . unwrap () ; } pair . drive () ; let client_stats = pair . client_conn_mut (client_ch) . stats () ; assert_eq ! (client_stats . udp_tx . ios , client_stats . udp_tx . datagrams) ; }
};
}

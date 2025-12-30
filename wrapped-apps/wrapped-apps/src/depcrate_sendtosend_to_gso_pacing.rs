// Generated macro for send_to_gso_pacing (function)
macro_rules! Depcrate_sendtosend_to_gso_pacing {
() => {
// Module: crate::sendto
// Provides: {"send_to_gso_pacing"}
// Dependencies: {}
# [doc = " For non-Linux platforms."] # [cfg (not (target_os = "linux"))] fn send_to_gso_pacing (_socket : & mio :: net :: UdpSocket , _buf : & [u8] , _send_info : & quiche :: SendInfo , _segment_size : usize ,) -> io :: Result < usize > { panic ! ("send_to_gso() should not be called on non-linux platforms") ; }
};
}

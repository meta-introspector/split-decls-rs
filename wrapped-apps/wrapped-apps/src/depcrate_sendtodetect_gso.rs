// Generated macro for detect_gso (function)
macro_rules! Depcrate_sendtodetect_gso {
() => {
// Module: crate::sendto
// Provides: {"detect_gso"}
// Dependencies: {}
# [doc = " For non-Linux, there is no GSO support."] # [cfg (not (target_os = "linux"))] pub fn detect_gso (_socket : & mio :: net :: UdpSocket , _segment_size : usize) -> bool { false }
};
}

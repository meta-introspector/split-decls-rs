// Generated macro for udp_transmit (function)
macro_rules! Depcrateudp_transmit {
() => {
// Module: crate
// Provides: {"udp_transmit"}
// Dependencies: {}
fn udp_transmit < 'a > (t : & proto :: Transmit , buffer : & 'a [u8]) -> udp :: Transmit < 'a > { udp :: Transmit { destination : t . destination , ecn : t . ecn . map (udp_ecn) , contents : buffer , segment_size : t . segment_size , src_ip : t . src_ip , } }
};
}

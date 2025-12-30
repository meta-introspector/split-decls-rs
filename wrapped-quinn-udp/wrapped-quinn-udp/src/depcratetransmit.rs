// Generated macro for Transmit (struct)
macro_rules! DepcrateTransmit {
() => {
// Module: crate
// Provides: {"Transmit"}
// Dependencies: {}
# [doc = " An outgoing packet"] # [derive (Debug , Clone)] pub struct Transmit < 'a > { # [doc = " The socket this datagram should be sent to"] pub destination : SocketAddr , # [doc = " Explicit congestion notification bits to set on the packet"] pub ecn : Option < EcnCodepoint > , # [doc = " Contents of the datagram"] pub contents : & 'a [u8] , # [doc = " The segment size if this transmission contains multiple datagrams."] # [doc = " This is `None` if the transmit only contains a single datagram"] pub segment_size : Option < usize > , # [doc = " Optional source IP address for the datagram"] pub src_ip : Option < IpAddr > , }
};
}

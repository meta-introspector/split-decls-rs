// Generated macro for Transmit (struct)
macro_rules! DepcrateTransmit {
() => {
// Module: crate
// Provides: {"Transmit"}
// Dependencies: {}
# [doc = " An outgoing packet"] # [derive (Debug)] # [must_use] pub struct Transmit { # [doc = " The socket this datagram should be sent to"] pub destination : SocketAddr , # [doc = " Explicit congestion notification bits to set on the packet"] pub ecn : Option < EcnCodepoint > , # [doc = " Amount of data written to the caller-supplied buffer"] pub size : usize , # [doc = " The segment size if this transmission contains multiple datagrams."] # [doc = " This is `None` if the transmit only contains a single datagram"] pub segment_size : Option < usize > , # [doc = " Optional source IP address for the datagram"] pub src_ip : Option < IpAddr > , }
};
}

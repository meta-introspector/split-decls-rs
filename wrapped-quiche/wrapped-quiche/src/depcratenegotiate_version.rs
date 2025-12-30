// Generated macro for negotiate_version (function)
macro_rules! Depcratenegotiate_version {
() => {
// Module: crate
// Provides: {"negotiate_version"}
// Dependencies: {}
# [doc = " Writes a version negotiation packet."] # [doc = ""] # [doc = " The `scid` and `dcid` parameters are the source connection ID and the"] # [doc = " destination connection ID extracted from the received client's Initial"] # [doc = " packet that advertises an unsupported version."] # [doc = ""] # [doc = " ## Examples:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # let mut buf = [0; 512];"] # [doc = " # let mut out = [0; 512];"] # [doc = " # let socket = std::net::UdpSocket::bind(\"127.0.0.1:0\").unwrap();"] # [doc = " let (len, src) = socket.recv_from(&mut buf).unwrap();"] # [doc = ""] # [doc = " let hdr ="] # [doc = "     quiche::Header::from_slice(&mut buf[..len], quiche::MAX_CONN_ID_LEN)?;"] # [doc = ""] # [doc = " if hdr.version != quiche::PROTOCOL_VERSION {"] # [doc = "     let len = quiche::negotiate_version(&hdr.scid, &hdr.dcid, &mut out)?;"] # [doc = "     socket.send_to(&out[..len], &src).unwrap();"] # [doc = " }"] # [doc = " # Ok::<(), quiche::Error>(())"] # [doc = " ```"] # [inline] pub fn negotiate_version (scid : & ConnectionId , dcid : & ConnectionId , out : & mut [u8] ,) -> Result < usize > { packet :: negotiate_version (scid , dcid , out) }
};
}

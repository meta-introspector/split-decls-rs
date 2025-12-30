// Generated macro for impl_828 (impl)
macro_rules! Depcrate_packetimpl_828 {
() => {
// Module: crate::packet
// Provides: {"impl_828"}
// Dependencies: {}
impl PartialEncode { pub (crate) fn finish (self , buf : & mut [u8] , header_crypto : & dyn crypto :: HeaderKey , crypto : Option < (u64 , & dyn crypto :: PacketKey) > ,) { let Self { header_len , pn , .. } = self ; let (pn_len , write_len) = match pn { Some ((pn_len , write_len)) => (pn_len , write_len) , None => return , } ; let pn_pos = header_len - pn_len ; if write_len { let len = buf . len () - header_len + pn_len ; assert ! (len < 2usize . pow (14)) ; let mut slice = & mut buf [pn_pos - 2 .. pn_pos] ; slice . put_u16 (len as u16 | (0b01 << 14)) ; } if let Some ((number , crypto)) = crypto { crypto . encrypt (number , buf , header_len) ; } debug_assert ! (pn_pos + 4 + header_crypto . sample_size () <= buf . len () , "packet must be padded to at least {} bytes for header protection sampling" , pn_pos + 4 + header_crypto . sample_size ()) ; header_crypto . encrypt (pn_pos , buf) ; } }
};
}

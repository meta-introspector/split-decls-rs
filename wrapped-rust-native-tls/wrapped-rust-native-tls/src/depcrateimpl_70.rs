// Generated macro for impl_70 (impl)
macro_rules! Depcrateimpl_70 {
() => {
// Module: crate
// Provides: {"impl_70"}
// Dependencies: {}
impl < S : io :: Read + io :: Write > TlsStream < S > { # [doc = " Returns the number of bytes that can be read without resulting in any"] # [doc = " network calls."] pub fn buffered_read_size (& self) -> Result < usize > { Ok (self . 0 . buffered_read_size () ?) } # [doc = " Returns the peer's leaf certificate, if available."] pub fn peer_certificate (& self) -> Result < Option < Certificate > > { Ok (self . 0 . peer_certificate () ? . map (Certificate)) } # [doc = " Returns the tls-server-end-point channel binding data as defined in [RFC 5929]."] # [doc = ""] # [doc = " [RFC 5929]: https://tools.ietf.org/html/rfc5929"] pub fn tls_server_end_point (& self) -> Result < Option < Vec < u8 > > > { Ok (self . 0 . tls_server_end_point () ?) } # [doc = " Returns the negotiated ALPN protocol."] # [cfg (feature = "alpn")] # [cfg_attr (docsrs , doc (cfg (feature = "alpn")))] pub fn negotiated_alpn (& self) -> Result < Option < Vec < u8 > > > { Ok (self . 0 . negotiated_alpn () ?) } # [doc = " Shuts down the TLS session."] pub fn shutdown (& mut self) -> io :: Result < () > { self . 0 . shutdown () ? ; Ok (()) } }
};
}

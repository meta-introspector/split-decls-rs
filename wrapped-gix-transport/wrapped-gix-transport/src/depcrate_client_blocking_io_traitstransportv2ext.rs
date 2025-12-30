// Generated macro for TransportV2Ext (trait)
macro_rules! Depcrate_client_blocking_io_traitsTransportV2Ext {
() => {
// Module: crate::client::blocking_io::traits
// Provides: {"TransportV2Ext"}
// Dependencies: {}
# [doc = " An extension trait to add more methods to everything implementing [`Transport`]."] pub trait TransportV2Ext { # [doc = " Invoke a protocol V2 style `command` with given `capabilities` and optional command specific `arguments`."] # [doc = " The `capabilities` were communicated during the handshake."] # [doc = " If `trace` is `true`, then all packetlines written and received will be traced using facilities provided by the `gix_trace` crate."] # [doc = ""] # [doc = " _Note:_ panics if [handshake][Transport::handshake()] wasn't performed beforehand."] fn invoke < 'a > (& mut self , command : & str , capabilities : impl Iterator < Item = (& 'a str , Option < impl AsRef < str > >) > + 'a , arguments : Option < impl Iterator < Item = bstr :: BString > > , trace : bool ,) -> Result < Box < dyn ExtendedBufRead < '_ > + Unpin + '_ > , Error > ; }
};
}

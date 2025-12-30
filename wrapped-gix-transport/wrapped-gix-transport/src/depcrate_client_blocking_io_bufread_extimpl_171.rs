// Generated macro for impl_171 (impl)
macro_rules! Depcrate_client_blocking_io_bufread_extimpl_171 {
() => {
// Module: crate::client::blocking_io::bufread_ext
// Provides: {"impl_171"}
// Dependencies: {}
impl < T : ReadlineBufRead + ? Sized > ReadlineBufRead for Box < T > { fn readline (& mut self) -> Option < io :: Result < Result < PacketLineRef < '_ > , gix_packetline :: decode :: Error > > > { ReadlineBufRead :: readline (self . deref_mut ()) } fn readline_str (& mut self , line : & mut String) -> io :: Result < usize > { ReadlineBufRead :: readline_str (self . deref_mut () , line) } }
};
}

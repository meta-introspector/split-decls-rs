// Generated macro for impl_23 (impl)
macro_rules! Depcrate_client_async_io_bufread_extimpl_23 {
() => {
// Module: crate::client::async_io::bufread_ext
// Provides: {"impl_23"}
// Dependencies: {}
# [async_trait (? Send)] impl < 'a , T : AsyncRead + Unpin > ReadlineBufRead for WithSidebands < 'a , T , HandleProgress < 'a > > { async fn readline (& mut self) -> Option < io :: Result < Result < PacketLineRef < '_ > , gix_packetline :: decode :: Error > > > { self . read_data_line () . await } async fn readline_str (& mut self , line : & mut String) -> io :: Result < usize > { self . read_line_to_string (line) . await } }
};
}

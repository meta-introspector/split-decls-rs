// Generated macro for impl_22 (impl)
macro_rules! Depcrate_client_async_io_bufread_extimpl_22 {
() => {
// Module: crate::client::async_io::bufread_ext
// Provides: {"impl_22"}
// Dependencies: {}
# [async_trait (? Send)] impl < T : AsyncRead + Unpin > ReadlineBufRead for WithSidebands < '_ , T , for < 'b > fn (bool , & 'b [u8]) -> ProgressAction > { async fn readline (& mut self) -> Option < io :: Result < Result < PacketLineRef < '_ > , gix_packetline :: decode :: Error > > > { self . read_data_line () . await } async fn readline_str (& mut self , line : & mut String) -> io :: Result < usize > { self . read_line_to_string (line) . await } }
};
}

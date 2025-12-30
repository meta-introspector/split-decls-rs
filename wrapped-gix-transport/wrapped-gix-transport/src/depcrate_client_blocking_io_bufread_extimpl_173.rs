// Generated macro for impl_173 (impl)
macro_rules! Depcrate_client_blocking_io_bufread_extimpl_173 {
() => {
// Module: crate::client::blocking_io::bufread_ext
// Provides: {"impl_173"}
// Dependencies: {}
impl < T : io :: Read > ReadlineBufRead for WithSidebands < '_ , T , fn (bool , & [u8]) -> ProgressAction > { fn readline (& mut self) -> Option < io :: Result < Result < PacketLineRef < '_ > , gix_packetline :: decode :: Error > > > { self . read_data_line () } fn readline_str (& mut self , line : & mut String) -> io :: Result < usize > { self . read_line_to_string (line) } }
};
}

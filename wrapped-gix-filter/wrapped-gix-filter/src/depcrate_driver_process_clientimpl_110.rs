// Generated macro for impl_110 (impl)
macro_rules! Depcrate_driver_process_clientimpl_110 {
() => {
// Module: crate::driver::process::client
// Provides: {"impl_110"}
// Dependencies: {}
impl std :: io :: Read for ReadProcessOutputAndStatus < '_ > { fn read (& mut self , buf : & mut [u8]) -> std :: io :: Result < usize > { let num_read = self . inner . read (buf) ? ; if num_read == 0 { self . inner . reset_with (& [gix_packetline :: PacketLineRef :: Flush]) ; let status = read_status (& mut self . inner) ? ; if status . is_success () { Ok (0) } else { Err (std :: io :: Error :: other (format ! ("Process indicated error after reading: {}" , status . message () . unwrap_or_default ()))) } } else { Ok (num_read) } } }
};
}

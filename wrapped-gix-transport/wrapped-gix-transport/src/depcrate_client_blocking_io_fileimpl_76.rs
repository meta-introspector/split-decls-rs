// Generated macro for impl_76 (impl)
macro_rules! Depcrate_client_blocking_io_fileimpl_76 {
() => {
// Module: crate::client::blocking_io::file
// Provides: {"impl_76"}
// Dependencies: {}
impl std :: io :: Read for ReadStdoutFailOnError { fn read (& mut self , buf : & mut [u8]) -> std :: io :: Result < usize > { let res = self . read . read (buf) ; self . swap_err_if_present_in_stderr (buf . len () , res) } }
};
}

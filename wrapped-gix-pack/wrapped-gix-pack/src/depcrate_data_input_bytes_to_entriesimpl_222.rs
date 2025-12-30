// Generated macro for impl_222 (impl)
macro_rules! Depcrate_data_input_bytes_to_entriesimpl_222 {
() => {
// Module: crate::data::input::bytes_to_entries
// Provides: {"impl_222"}
// Dependencies: {}
impl < R , W > io :: Read for PassThrough < R , W > where W : io :: Write , R : io :: Read , { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let bytes_read = self . read . read (buf) ? ; self . write . write_all (& buf [.. bytes_read]) ? ; Ok (bytes_read) } }
};
}

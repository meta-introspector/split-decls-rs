// Generated macro for impl_227 (impl)
macro_rules! Depcrate_data_input_bytes_to_entriesimpl_227 {
() => {
// Module: crate::data::input::bytes_to_entries
// Provides: {"impl_227"}
// Dependencies: {}
impl < T > std :: io :: Write for HashWrite < '_ , T > where T : std :: io :: Write , { fn write (& mut self , buf : & [u8]) -> std :: io :: Result < usize > { let written = self . inner . write (buf) ? ; self . hash . update (& buf [.. written]) ; Ok (written) } fn flush (& mut self) -> std :: io :: Result < () > { self . inner . flush () } }
};
}

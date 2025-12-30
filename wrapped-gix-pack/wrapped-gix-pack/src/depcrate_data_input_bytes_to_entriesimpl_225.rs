// Generated macro for impl_225 (impl)
macro_rules! Depcrate_data_input_bytes_to_entriesimpl_225 {
() => {
// Module: crate::data::input::bytes_to_entries
// Provides: {"impl_225"}
// Dependencies: {}
impl < R > io :: Read for DecompressRead < '_ , R > where R : io :: BufRead , { fn read (& mut self , into : & mut [u8]) -> io :: Result < usize > { gix_features :: zlib :: stream :: inflate :: read (& mut self . inner , self . decompressor , into) } }
};
}

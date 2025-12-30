// Generated macro for impl_425 (impl)
macro_rules! Depcrateimpl_425 {
() => {
// Module: crate
// Provides: {"impl_425"}
// Dependencies: {}
# [cfg (feature = "encoder")] impl < W : Write > Write for CountingWriter < W > { fn write (& mut self , buf : & [u8]) -> Result < usize > { let bytes_written = self . inner . write (buf) ? ; self . bytes_written += bytes_written as u64 ; Ok (bytes_written) } fn flush (& mut self) -> Result < () > { self . inner . flush () } }
};
}

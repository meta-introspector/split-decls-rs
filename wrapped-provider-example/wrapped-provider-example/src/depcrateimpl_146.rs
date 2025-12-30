// Generated macro for impl_146 (impl)
macro_rules! Depcrateimpl_146 {
() => {
// Module: crate
// Provides: {"impl_146"}
// Dependencies: {}
impl SecureRandom for Provider { fn fill (& self , bytes : & mut [u8]) -> Result < () , GetRandomFailed > { use rand_core :: RngCore ; rand_core :: OsRng . try_fill_bytes (bytes) . map_err (| _ | GetRandomFailed) } }
};
}

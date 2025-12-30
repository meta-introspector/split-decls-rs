// Generated macro for impl_99 (impl)
macro_rules! Depcrateimpl_99 {
() => {
// Module: crate
// Provides: {"impl_99"}
// Dependencies: {}
impl SecureRandom for NotRandom { fn fill (& self , buf : & mut [u8]) -> Result < () , GetRandomFailed > { buf . fill (0x5a) ; Ok (()) } }
};
}

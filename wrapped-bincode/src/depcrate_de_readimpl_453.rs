// Generated macro for impl_453 (impl)
macro_rules! Depcrate_de_readimpl_453 {
() => {
// Module: crate::de::read
// Provides: {"impl_453"}
// Dependencies: {}
impl < T > Reader for & mut T where T : Reader , { # [inline] fn read (& mut self , bytes : & mut [u8]) -> Result < () , DecodeError > { (* * self) . read (bytes) } # [inline] fn peek_read (& mut self , n : usize) -> Option < & [u8] > { (* * self) . peek_read (n) } # [inline] fn consume (& mut self , n : usize) { (* self) . consume (n) } }
};
}

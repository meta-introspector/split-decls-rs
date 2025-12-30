// Generated macro for impl_262 (impl)
macro_rules! Depcrate_writerimpl_262 {
() => {
// Module: crate::writer
// Provides: {"impl_262"}
// Dependencies: {}
# [cfg (feature = "std")] impl < W : io :: Write > Writer for W { fn write (& mut self , slice : & [u8]) -> Result < () > { < Self as io :: Write > :: write (self , slice) ? ; Ok (()) } }
};
}

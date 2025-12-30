// Generated macro for impl_235 (impl)
macro_rules! Depcrate_serimpl_235 {
() => {
// Module: crate::ser
// Provides: {"impl_235"}
// Dependencies: {}
impl BorshSerialize for u8 { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { writer . write_all (core :: slice :: from_ref (self)) } # [inline] fn u8_slice (slice : & [Self]) -> Option < & [u8] > { Some (slice) } }
};
}

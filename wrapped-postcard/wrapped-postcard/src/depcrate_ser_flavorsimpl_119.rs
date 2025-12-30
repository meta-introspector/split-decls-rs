// Generated macro for impl_119 (impl)
macro_rules! Depcrate_ser_flavorsimpl_119 {
() => {
// Module: crate::ser::flavors
// Provides: {"impl_119"}
// Dependencies: {}
impl Flavor for Size { type Output = usize ; # [inline (always)] fn try_push (& mut self , _b : u8) -> Result < () > { self . size += 1 ; Ok (()) } # [inline (always)] fn try_extend (& mut self , b : & [u8]) -> Result < () > { self . size += b . len () ; Ok (()) } fn finalize (self) -> Result < Self :: Output > { Ok (self . size) } }
};
}

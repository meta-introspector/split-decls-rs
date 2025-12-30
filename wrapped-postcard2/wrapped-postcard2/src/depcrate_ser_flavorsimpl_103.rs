// Generated macro for impl_103 (impl)
macro_rules! Depcrate_ser_flavorsimpl_103 {
() => {
// Module: crate::ser::flavors
// Provides: {"impl_103"}
// Dependencies: {}
impl < T > Flavor for ExtendFlavor < T > where T : core :: iter :: Extend < u8 > , { type Output = T ; # [inline (always)] fn try_push (& mut self , data : u8) -> Result < () > { self . iter . extend ([data]) ; Ok (()) } # [inline (always)] fn try_extend (& mut self , b : & [u8]) -> Result < () > { self . iter . extend (b . iter () . copied ()) ; Ok (()) } fn finalize (self) -> Result < Self :: Output > { Ok (self . iter) } }
};
}

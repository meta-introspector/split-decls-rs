// Generated macro for impl_322 (impl)
macro_rules! Depcrate_serimpl_322 {
() => {
// Module: crate::ser
// Provides: {"impl_322"}
// Dependencies: {}
impl < T : ? Sized > BorshSerialize for PhantomData < T > { fn serialize < W : Write > (& self , _ : & mut W) -> Result < () > { Ok (()) } }
};
}

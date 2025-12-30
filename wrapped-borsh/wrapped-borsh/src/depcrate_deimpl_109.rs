// Generated macro for impl_109 (impl)
macro_rules! Depcrate_deimpl_109 {
() => {
// Module: crate::de
// Provides: {"impl_109"}
// Dependencies: {}
impl < T : ? Sized > BorshDeserialize for PhantomData < T > { fn deserialize_reader < R : Read > (_ : & mut R) -> Result < Self > { Ok (PhantomData) } }
};
}

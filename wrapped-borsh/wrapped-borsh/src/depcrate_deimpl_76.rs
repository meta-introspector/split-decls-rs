// Generated macro for impl_76 (impl)
macro_rules! Depcrate_deimpl_76 {
() => {
// Module: crate::de
// Provides: {"impl_76"}
// Dependencies: {}
impl < T , U > BorshDeserialize for Box < T > where U : Into < Box < T > > + Borrow < T > , T : ToOwned < Owned = U > + ? Sized , T :: Owned : BorshDeserialize , { fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { Ok (T :: Owned :: deserialize_reader (reader) ? . into ()) } }
};
}

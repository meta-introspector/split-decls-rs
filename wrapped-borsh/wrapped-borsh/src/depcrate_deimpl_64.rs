// Generated macro for impl_64 (impl)
macro_rules! Depcrate_deimpl_64 {
() => {
// Module: crate::de
// Provides: {"impl_64"}
// Dependencies: {}
impl < T > BorshDeserialize for Cow < '_ , T > where T : ToOwned + ? Sized , T :: Owned : BorshDeserialize , { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { Ok (Cow :: Owned (BorshDeserialize :: deserialize_reader (reader) ?)) } }
};
}

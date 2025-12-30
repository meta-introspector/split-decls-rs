// Generated macro for impl_65 (impl)
macro_rules! Depcrate_deimpl_65 {
() => {
// Module: crate::de
// Provides: {"impl_65"}
// Dependencies: {}
impl < T > BorshDeserialize for VecDeque < T > where T : BorshDeserialize , { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { let vec = < Vec < T > > :: deserialize_reader (reader) ? ; Ok (vec . into ()) } }
};
}

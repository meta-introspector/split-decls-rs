// Generated macro for impl_66 (impl)
macro_rules! Depcrate_deimpl_66 {
() => {
// Module: crate::de
// Provides: {"impl_66"}
// Dependencies: {}
impl < T > BorshDeserialize for LinkedList < T > where T : BorshDeserialize , { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { let vec = < Vec < T > > :: deserialize_reader (reader) ? ; Ok (vec . into_iter () . collect :: < LinkedList < T > > ()) } }
};
}

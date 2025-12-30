// Generated macro for impl_110 (impl)
macro_rules! Depcrate_deimpl_110 {
() => {
// Module: crate::de
// Provides: {"impl_110"}
// Dependencies: {}
impl < T > BorshDeserialize for core :: cell :: Cell < T > where T : BorshDeserialize + Copy , { fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { < T as BorshDeserialize > :: deserialize_reader (reader) . map (core :: cell :: Cell :: new) } }
};
}

// Generated macro for impl_111 (impl)
macro_rules! Depcrate_deimpl_111 {
() => {
// Module: crate::de
// Provides: {"impl_111"}
// Dependencies: {}
impl < T > BorshDeserialize for core :: cell :: RefCell < T > where T : BorshDeserialize , { fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { < T as BorshDeserialize > :: deserialize_reader (reader) . map (core :: cell :: RefCell :: new) } }
};
}

// Generated macro for impl_323 (impl)
macro_rules! Depcrate_serimpl_323 {
() => {
// Module: crate::ser
// Provides: {"impl_323"}
// Dependencies: {}
impl < T > BorshSerialize for core :: cell :: Cell < T > where T : BorshSerialize + Copy , { fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { < T as BorshSerialize > :: serialize (& self . get () , writer) } }
};
}

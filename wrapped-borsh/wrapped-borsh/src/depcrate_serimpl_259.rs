// Generated macro for impl_259 (impl)
macro_rules! Depcrate_serimpl_259 {
() => {
// Module: crate::ser
// Provides: {"impl_259"}
// Dependencies: {}
impl BorshSerialize for usize { fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { BorshSerialize :: serialize (& (* self as u64) , writer) } }
};
}

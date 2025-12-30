// Generated macro for impl_258 (impl)
macro_rules! Depcrate_serimpl_258 {
() => {
// Module: crate::ser
// Provides: {"impl_258"}
// Dependencies: {}
impl BorshSerialize for isize { fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { BorshSerialize :: serialize (& (* self as i64) , writer) } }
};
}

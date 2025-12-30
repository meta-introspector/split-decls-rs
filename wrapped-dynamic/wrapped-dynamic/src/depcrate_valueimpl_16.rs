// Generated macro for impl_16 (impl)
macro_rules! Depcrate_valueimpl_16 {
() => {
// Module: crate::value
// Provides: {"impl_16"}
// Dependencies: {}
impl < T : sval :: Value > private :: EraseValue for T { fn erase_value (& self) -> crate :: private :: Erased < & dyn private :: DispatchValue > { crate :: private :: Erased (self) } }
};
}

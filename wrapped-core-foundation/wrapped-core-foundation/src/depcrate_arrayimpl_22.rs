// Generated macro for impl_22 (impl)
macro_rules! Depcrate_arrayimpl_22 {
() => {
// Module: crate::array
// Provides: {"impl_22"}
// Dependencies: {}
impl < T : FromVoid > ExactSizeIterator for CFArrayIterator < '_ , T > { fn len (& self) -> usize { (self . array . len () - self . index) as usize } }
};
}

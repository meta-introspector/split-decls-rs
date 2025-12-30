// Generated macro for impl_1329 (impl)
macro_rules! Depcrate_optionimpl_1329 {
() => {
// Module: crate::option
// Provides: {"impl_1329"}
// Dependencies: {}
impl < T : Strategy > Clone for OptionValueTree < T > where T :: Tree : Clone , { fn clone (& self) -> Self { OptionValueTree (self . 0 . clone ()) } }
};
}

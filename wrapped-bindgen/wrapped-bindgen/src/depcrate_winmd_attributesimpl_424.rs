// Generated macro for impl_424 (impl)
macro_rules! Depcrate_winmd_attributesimpl_424 {
() => {
// Module: crate::winmd::attributes
// Provides: {"impl_424"}
// Dependencies: {}
impl TypeAttributes { pub const ExplicitLayout : Self = Self (0x10) ; pub const WindowsRuntime : Self = Self (0x4000) ; pub fn is_nested (& self) -> bool { (self . 0 & 0x00000006) != 0 } }
};
}

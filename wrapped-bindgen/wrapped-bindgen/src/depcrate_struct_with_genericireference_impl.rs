// Generated macro for IReference_Impl (trait)
macro_rules! Depcrate_struct_with_genericIReference_Impl {
() => {
// Module: crate::struct_with_generic
// Provides: {"IReference_Impl"}
// Dependencies: {}
pub trait IReference_Impl < T > : IPropertyValue_Impl where T : windows_core :: RuntimeType + 'static , { fn Value (& self) -> windows_core :: Result < T > ; }
};
}

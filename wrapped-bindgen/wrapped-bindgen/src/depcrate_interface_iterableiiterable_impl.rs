// Generated macro for IIterable_Impl (trait)
macro_rules! Depcrate_interface_iterableIIterable_Impl {
() => {
// Module: crate::interface_iterable
// Provides: {"IIterable_Impl"}
// Dependencies: {}
pub trait IIterable_Impl < T > : windows_core :: IUnknownImpl where T : windows_core :: RuntimeType + 'static , { fn First (& self) -> windows_core :: Result < IIterator < T > > ; }
};
}

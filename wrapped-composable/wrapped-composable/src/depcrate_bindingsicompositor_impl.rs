// Generated macro for ICompositor_Impl (trait)
macro_rules! Depcrate_bindingsICompositor_Impl {
() => {
// Module: crate::bindings
// Provides: {"ICompositor_Impl"}
// Dependencies: {}
pub trait ICompositor_Impl : windows_core :: IUnknownImpl { fn CreateSpriteVisual (& self , brush : i32) -> windows_core :: Result < SpriteVisual > ; fn CreateContainerVisual (& self , children : i32) -> windows_core :: Result < ContainerVisual > ; }
};
}

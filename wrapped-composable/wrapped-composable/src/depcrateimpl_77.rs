// Generated macro for impl_77 (impl)
macro_rules! Depcrateimpl_77 {
() => {
// Module: crate
// Provides: {"impl_77"}
// Dependencies: {}
impl bindings :: ICompositor_Impl for Compositor_Impl { fn CreateSpriteVisual (& self , brush : i32) -> Result < bindings :: SpriteVisual > { Ok (SpriteVisual :: new (ContainerVisual :: new (self . to_object () , brush * 2) , brush) . into ()) } fn CreateContainerVisual (& self , children : i32) -> Result < bindings :: ContainerVisual > { Ok (ContainerVisual :: new (self . to_object () , children) . into ()) } }
};
}

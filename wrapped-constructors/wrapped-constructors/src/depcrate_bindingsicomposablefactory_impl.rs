// Generated macro for IComposableFactory_Impl (trait)
macro_rules! Depcrate_bindingsIComposableFactory_Impl {
() => {
// Module: crate::bindings
// Provides: {"IComposableFactory_Impl"}
// Dependencies: {}
pub trait IComposableFactory_Impl : windows_core :: IUnknownImpl { fn CreateInstance (& self , baseInterface : windows_core :: Ref < windows_core :: IInspectable > , innerInterface : windows_core :: OutRef < windows_core :: IInspectable > ,) -> windows_core :: Result < Composable > ; fn WithValue (& self , arg : i32 , baseInterface : windows_core :: Ref < windows_core :: IInspectable > , innerInterface : windows_core :: OutRef < windows_core :: IInspectable > ,) -> windows_core :: Result < Composable > ; }
};
}

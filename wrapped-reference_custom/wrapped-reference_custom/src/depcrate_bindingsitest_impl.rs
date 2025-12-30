// Generated macro for ITest_Impl (trait)
macro_rules! Depcrate_bindingsITest_Impl {
() => {
// Module: crate::bindings
// Provides: {"ITest_Impl"}
// Dependencies: {}
pub trait ITest_Impl : windows_core :: IUnknownImpl { fn Numerics (& self , n : & Vector2) -> windows_core :: Result < () > ; fn Collections (& self , c : windows_core :: Ref < IVector < i32 > >) -> windows_core :: Result < () > ; fn Async (& self) -> windows_core :: Result < IAsyncAction > ; fn Windows (& self , s : windows_core :: Ref < windows :: Foundation :: IStringable > ,) -> windows_core :: Result < () > ; }
};
}

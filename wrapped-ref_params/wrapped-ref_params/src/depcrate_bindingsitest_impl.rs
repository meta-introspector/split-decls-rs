// Generated macro for ITest_Impl (trait)
macro_rules! Depcrate_bindingsITest_Impl {
() => {
// Module: crate::bindings
// Provides: {"ITest_Impl"}
// Dependencies: {}
pub trait ITest_Impl : windows_core :: IUnknownImpl { fn Input (& self , input : windows_core :: Ref < ITest >) -> windows_core :: Result < i32 > ; fn Output (& self , value : i32 , output : windows_core :: OutRef < ITest >) -> windows_core :: Result < () > ; fn Current (& self) -> windows_core :: Result < i32 > ; fn SetCurrent (& self , value : i32) -> windows_core :: Result < () > ; }
};
}

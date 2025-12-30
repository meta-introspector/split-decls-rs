// Generated macro for IClass_Impl (trait)
macro_rules! Depcrate_bindingsIClass_Impl {
() => {
// Module: crate::bindings
// Provides: {"IClass_Impl"}
// Dependencies: {}
pub trait IClass_Impl : windows_core :: IUnknownImpl { fn Signal (& self , value : i32) -> windows_core :: Result < i32 > ; fn Event (& self , handler : windows_core :: Ref < windows :: Foundation :: TypedEventHandler < Class , i32 > > ,) -> windows_core :: Result < i64 > ; fn RemoveEvent (& self , token : i64) -> windows_core :: Result < () > ; }
};
}

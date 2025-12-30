// Generated macro for IClassStatics_Impl (trait)
macro_rules! Depcrate_bindingsIClassStatics_Impl {
() => {
// Module: crate::bindings
// Provides: {"IClassStatics_Impl"}
// Dependencies: {}
pub trait IClassStatics_Impl : windows_core :: IUnknownImpl { fn StaticSignal (& self , value : i32) -> windows_core :: Result < i32 > ; fn StaticEvent (& self , handler : windows_core :: Ref < windows :: Foundation :: EventHandler < i32 > > ,) -> windows_core :: Result < i64 > ; fn RemoveStaticEvent (& self , token : i64) -> windows_core :: Result < () > ; }
};
}

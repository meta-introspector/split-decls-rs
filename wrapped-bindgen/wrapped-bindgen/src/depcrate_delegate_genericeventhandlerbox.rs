// Generated macro for EventHandlerBox (struct)
macro_rules! Depcrate_delegate_genericEventHandlerBox {
() => {
// Module: crate::delegate_generic
// Provides: {"EventHandlerBox"}
// Dependencies: {}
# [repr (C)] struct EventHandlerBox < T , F : Fn (windows_core :: Ref < windows_core :: IInspectable > , windows_core :: Ref < T > ,) -> windows_core :: Result < () > + Send + 'static , > where T : windows_core :: RuntimeType + 'static , { vtable : * const EventHandler_Vtbl < T > , invoke : F , count : windows_core :: imp :: RefCount , }
};
}

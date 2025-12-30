// Generated macro for impl_227 (impl)
macro_rules! Depcrate_delegate_genericimpl_227 {
() => {
// Module: crate::delegate_generic
// Provides: {"impl_227"}
// Dependencies: {}
impl < T : windows_core :: RuntimeType + 'static > EventHandler < T > { pub fn new < F : Fn (windows_core :: Ref < windows_core :: IInspectable > , windows_core :: Ref < T > ,) -> windows_core :: Result < () > + Send + 'static , > (invoke : F ,) -> Self { let com = EventHandlerBox { vtable : & EventHandlerBox :: < T , F > :: VTABLE , count : windows_core :: imp :: RefCount :: new (1) , invoke , } ; unsafe { core :: mem :: transmute (windows_core :: imp :: Box :: new (com)) } } pub fn Invoke < P0 , P1 > (& self , sender : P0 , args : P1) -> windows_core :: Result < () > where P0 : windows_core :: Param < windows_core :: IInspectable > , P1 : windows_core :: Param < T > , { let this = self ; unsafe { (windows_core :: Interface :: vtable (this) . Invoke) (windows_core :: Interface :: as_raw (this) , sender . param () . abi () , args . param () . abi () ,) . ok () } } }
};
}

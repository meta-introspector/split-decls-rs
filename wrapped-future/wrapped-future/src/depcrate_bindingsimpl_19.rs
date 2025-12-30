// Generated macro for impl_19 (impl)
macro_rules! Depcrate_bindingsimpl_19 {
() => {
// Module: crate::bindings
// Provides: {"impl_19"}
// Dependencies: {}
impl < TProgress : windows_core :: RuntimeType + 'static > AsyncActionWithProgressCompletedHandler < TProgress > { pub fn new < F : Fn (windows_core :: Ref < IAsyncActionWithProgress < TProgress > > , AsyncStatus ,) -> windows_core :: Result < () > + Send + 'static , > (invoke : F ,) -> Self { let com = AsyncActionWithProgressCompletedHandlerBox { vtable : & AsyncActionWithProgressCompletedHandlerBox :: < TProgress , F > :: VTABLE , count : windows_core :: imp :: RefCount :: new (1) , invoke , } ; unsafe { core :: mem :: transmute (windows_core :: imp :: Box :: new (com)) } } pub fn Invoke < P0 > (& self , asyncinfo : P0 , asyncstatus : AsyncStatus) -> windows_core :: Result < () > where P0 : windows_core :: Param < IAsyncActionWithProgress < TProgress > > , { let this = self ; unsafe { (windows_core :: Interface :: vtable (this) . Invoke) (windows_core :: Interface :: as_raw (this) , asyncinfo . param () . abi () , asyncstatus ,) . ok () } } }
};
}

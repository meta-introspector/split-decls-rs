// Generated macro for impl_33 (impl)
macro_rules! Depcrate_bindingsimpl_33 {
() => {
// Module: crate::bindings
// Provides: {"impl_33"}
// Dependencies: {}
impl < TResult : windows_core :: RuntimeType + 'static , TProgress : windows_core :: RuntimeType + 'static , > AsyncOperationProgressHandler < TResult , TProgress > { pub fn new < F : Fn (windows_core :: Ref < IAsyncOperationWithProgress < TResult , TProgress > > , windows_core :: Ref < TProgress > ,) -> windows_core :: Result < () > + Send + 'static , > (invoke : F ,) -> Self { let com = AsyncOperationProgressHandlerBox { vtable : & AsyncOperationProgressHandlerBox :: < TResult , TProgress , F > :: VTABLE , count : windows_core :: imp :: RefCount :: new (1) , invoke , } ; unsafe { core :: mem :: transmute (windows_core :: imp :: Box :: new (com)) } } pub fn Invoke < P0 , P1 > (& self , asyncinfo : P0 , progressinfo : P1) -> windows_core :: Result < () > where P0 : windows_core :: Param < IAsyncOperationWithProgress < TResult , TProgress > > , P1 : windows_core :: Param < TProgress > , { let this = self ; unsafe { (windows_core :: Interface :: vtable (this) . Invoke) (windows_core :: Interface :: as_raw (this) , asyncinfo . param () . abi () , progressinfo . param () . abi () ,) . ok () } } }
};
}

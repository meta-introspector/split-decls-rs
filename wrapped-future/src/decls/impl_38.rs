macro_rules! deps {
    () => {
        AsyncStatus!();
        IAsyncOperationWithProgress!();
        AsyncOperationWithProgressCompletedHandlerBox!();
        AsyncOperationWithProgressCompletedHandler!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < TResult : windows_core :: RuntimeType + 'static , TProgress : windows_core :: RuntimeType + 'static , > AsyncOperationWithProgressCompletedHandler < TResult , TProgress > { pub fn new < F : Fn (windows_core :: Ref < IAsyncOperationWithProgress < TResult , TProgress > > , AsyncStatus ,) -> windows_core :: Result < () > + Send + 'static , > (invoke : F ,) -> Self { let com = AsyncOperationWithProgressCompletedHandlerBox { vtable : & AsyncOperationWithProgressCompletedHandlerBox :: < TResult , TProgress , F > :: VTABLE , count : windows_core :: imp :: RefCount :: new (1) , invoke , } ; unsafe { core :: mem :: transmute (windows_core :: imp :: Box :: new (com)) } } pub fn Invoke < P0 > (& self , asyncinfo : P0 , asyncstatus : AsyncStatus) -> windows_core :: Result < () > where P0 : windows_core :: Param < IAsyncOperationWithProgress < TResult , TProgress > > , { let this = self ; unsafe { (windows_core :: Interface :: vtable (this) . Invoke) (windows_core :: Interface :: as_raw (this) , asyncinfo . param () . abi () , asyncstatus ,) . ok () } } }
    };
}

impl_38!();
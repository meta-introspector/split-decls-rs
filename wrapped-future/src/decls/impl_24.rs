macro_rules! deps {
    () => {
        AsyncStatus!();
        IAsyncOperation!();
        AsyncOperationCompletedHandlerBox!();
        AsyncOperationCompletedHandler!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < TResult : windows_core :: RuntimeType + 'static > AsyncOperationCompletedHandler < TResult > { pub fn new < F : Fn (windows_core :: Ref < IAsyncOperation < TResult > > , AsyncStatus) -> windows_core :: Result < () > + Send + 'static , > (invoke : F ,) -> Self { let com = AsyncOperationCompletedHandlerBox { vtable : & AsyncOperationCompletedHandlerBox :: < TResult , F > :: VTABLE , count : windows_core :: imp :: RefCount :: new (1) , invoke , } ; unsafe { core :: mem :: transmute (windows_core :: imp :: Box :: new (com)) } } pub fn Invoke < P0 > (& self , asyncinfo : P0 , asyncstatus : AsyncStatus) -> windows_core :: Result < () > where P0 : windows_core :: Param < IAsyncOperation < TResult > > , { let this = self ; unsafe { (windows_core :: Interface :: vtable (this) . Invoke) (windows_core :: Interface :: as_raw (this) , asyncinfo . param () . abi () , asyncstatus ,) . ok () } } }
    };
}

impl_24!()
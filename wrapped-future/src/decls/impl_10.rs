macro_rules! deps {
    () => {
        AsyncActionProgressHandler!();
        AsyncActionProgressHandlerBox!();
        IAsyncActionWithProgress!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < TProgress : windows_core :: RuntimeType + 'static > AsyncActionProgressHandler < TProgress > { pub fn new < F : Fn (windows_core :: Ref < IAsyncActionWithProgress < TProgress > > , windows_core :: Ref < TProgress > ,) -> windows_core :: Result < () > + Send + 'static , > (invoke : F ,) -> Self { let com = AsyncActionProgressHandlerBox { vtable : & AsyncActionProgressHandlerBox :: < TProgress , F > :: VTABLE , count : windows_core :: imp :: RefCount :: new (1) , invoke , } ; unsafe { core :: mem :: transmute (windows_core :: imp :: Box :: new (com)) } } pub fn Invoke < P0 , P1 > (& self , asyncinfo : P0 , progressinfo : P1) -> windows_core :: Result < () > where P0 : windows_core :: Param < IAsyncActionWithProgress < TProgress > > , P1 : windows_core :: Param < TProgress > , { let this = self ; unsafe { (windows_core :: Interface :: vtable (this) . Invoke) (windows_core :: Interface :: as_raw (this) , asyncinfo . param () . abi () , progressinfo . param () . abi () ,) . ok () } } }
    };
}

impl_10!();
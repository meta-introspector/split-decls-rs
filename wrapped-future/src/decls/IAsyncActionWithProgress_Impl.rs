macro_rules! deps {
    () => {
        AsyncActionWithProgressCompletedHandler!();
        AsyncActionProgressHandler!();
        IAsyncInfo_Impl!();
    };
}

macro_rules! IAsyncActionWithProgress_Impl {
    () => {
        deps!();
        pub trait IAsyncActionWithProgress_Impl < TProgress > : IAsyncInfo_Impl where TProgress : windows_core :: RuntimeType + 'static , { fn SetProgress (& self , handler : windows_core :: Ref < AsyncActionProgressHandler < TProgress > > ,) -> windows_core :: Result < () > ; fn Progress (& self) -> windows_core :: Result < AsyncActionProgressHandler < TProgress > > ; fn SetCompleted (& self , handler : windows_core :: Ref < AsyncActionWithProgressCompletedHandler < TProgress > > ,) -> windows_core :: Result < () > ; fn Completed (& self) -> windows_core :: Result < AsyncActionWithProgressCompletedHandler < TProgress > > ; fn GetResults (& self) -> windows_core :: Result < () > ; }
    };
}

IAsyncActionWithProgress_Impl!()
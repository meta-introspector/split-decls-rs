macro_rules! deps {
    () => {
        AsyncOperationCompletedHandler!();
        IAsyncInfo_Impl!();
    };
}

macro_rules! IAsyncOperation_Impl {
    () => {
        deps!();
        pub trait IAsyncOperation_Impl < TResult > : IAsyncInfo_Impl where TResult : windows_core :: RuntimeType + 'static , { fn SetCompleted (& self , handler : windows_core :: Ref < AsyncOperationCompletedHandler < TResult > > ,) -> windows_core :: Result < () > ; fn Completed (& self) -> windows_core :: Result < AsyncOperationCompletedHandler < TResult > > ; fn GetResults (& self) -> windows_core :: Result < TResult > ; }
    };
}

IAsyncOperation_Impl!()
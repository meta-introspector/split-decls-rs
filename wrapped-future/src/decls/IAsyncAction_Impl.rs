macro_rules! deps {
    () => {
        IAsyncInfo_Impl!();
    };
}

macro_rules! IAsyncAction_Impl {
    () => {
        deps!();
        pub trait IAsyncAction_Impl : IAsyncInfo_Impl { fn SetCompleted (& self , handler : windows_core :: Ref < AsyncActionCompletedHandler > ,) -> windows_core :: Result < () > ; fn Completed (& self) -> windows_core :: Result < AsyncActionCompletedHandler > ; fn GetResults (& self) -> windows_core :: Result < () > ; }
    };
}

IAsyncAction_Impl!()
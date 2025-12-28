macro_rules! deps {
    () => {
        AsyncOperationCompletedHandler!();
        IAsyncOperation_Impl!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl < T : RuntimeType > IAsyncOperation_Impl < T > for ReadyOperation_Impl < T > { fn SetCompleted (& self , handler : Ref < AsyncOperationCompletedHandler < T > >) -> Result < () > { self . 0 . invoke_completed (& self . as_interface () , handler) } fn Completed (& self) -> Result < AsyncOperationCompletedHandler < T > > { Err (Error :: empty ()) } fn GetResults (& self) -> Result < T > { self . 0 . result . clone () } }
    };
}

impl_144!();
macro_rules! deps {
    () => {
        IAsyncOperation_Impl!();
        AsyncOperationCompletedHandler!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl < T : RuntimeType > IAsyncOperation_Impl < T > for Operation_Impl < T > { fn SetCompleted (& self , handler : Ref < AsyncOperationCompletedHandler < T > >) -> Result < () > { self . 0 . set_completed (& self . as_interface () , handler) } fn Completed (& self) -> Result < AsyncOperationCompletedHandler < T > > { Err (Error :: empty ()) } fn GetResults (& self) -> Result < T > { self . 0 . get_results () } }
    };
}

impl_166!()
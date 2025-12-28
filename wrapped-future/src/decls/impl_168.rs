macro_rules! deps {
    () => {
        IAsyncOperationWithProgress_Impl!();
        AsyncOperationWithProgressCompletedHandler!();
        AsyncOperationProgressHandler!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < T : RuntimeType , P : RuntimeType > IAsyncOperationWithProgress_Impl < T , P > for OperationWithProgress_Impl < T , P > { fn SetCompleted (& self , handler : Ref < AsyncOperationWithProgressCompletedHandler < T , P > > ,) -> Result < () > { self . 0 . set_completed (& self . as_interface () , handler) } fn Completed (& self) -> Result < AsyncOperationWithProgressCompletedHandler < T , P > > { Err (Error :: empty ()) } fn GetResults (& self) -> Result < T > { self . 0 . get_results () } fn SetProgress (& self , _ : Ref < AsyncOperationProgressHandler < T , P > >) -> Result < () > { Ok (()) } fn Progress (& self) -> Result < AsyncOperationProgressHandler < T , P > > { Err (Error :: empty ()) } }
    };
}

impl_168!()
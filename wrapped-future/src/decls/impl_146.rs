macro_rules! deps {
    () => {
        IAsyncOperationWithProgress_Impl!();
        AsyncOperationWithProgressCompletedHandler!();
        AsyncOperationProgressHandler!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl < T : RuntimeType , P : RuntimeType > IAsyncOperationWithProgress_Impl < T , P > for ReadyOperationWithProgress_Impl < T , P > { fn SetCompleted (& self , handler : Ref < AsyncOperationWithProgressCompletedHandler < T , P > > ,) -> Result < () > { self . 0 . invoke_completed (& self . as_interface () , handler) } fn Completed (& self) -> Result < AsyncOperationWithProgressCompletedHandler < T , P > > { Err (Error :: empty ()) } fn GetResults (& self) -> Result < T > { self . 0 . result . clone () } fn SetProgress (& self , _ : Ref < AsyncOperationProgressHandler < T , P > >) -> Result < () > { Ok (()) } fn Progress (& self) -> Result < AsyncOperationProgressHandler < T , P > > { Err (Error :: empty ()) } }
    };
}

impl_146!()
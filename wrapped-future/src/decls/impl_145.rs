macro_rules! deps {
    () => {
        IAsyncActionWithProgress_Impl!();
        AsyncActionProgressHandler!();
        AsyncActionWithProgressCompletedHandler!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < P : RuntimeType > IAsyncActionWithProgress_Impl < P > for ReadyActionWithProgress_Impl < P > { fn SetCompleted (& self , handler : Ref < AsyncActionWithProgressCompletedHandler < P > >) -> Result < () > { self . 0 . invoke_completed (& self . as_interface () , handler) } fn Completed (& self) -> Result < AsyncActionWithProgressCompletedHandler < P > > { Err (Error :: empty ()) } fn GetResults (& self) -> Result < () > { self . 0 . result . clone () } fn SetProgress (& self , _ : Ref < AsyncActionProgressHandler < P > >) -> Result < () > { Ok (()) } fn Progress (& self) -> Result < AsyncActionProgressHandler < P > > { Err (Error :: empty ()) } }
    };
}

impl_145!();
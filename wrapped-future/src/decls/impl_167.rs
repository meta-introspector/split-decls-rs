macro_rules! deps {
    () => {
        AsyncActionProgressHandler!();
        IAsyncActionWithProgress_Impl!();
        AsyncActionWithProgressCompletedHandler!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < P : RuntimeType > IAsyncActionWithProgress_Impl < P > for ActionWithProgress_Impl < P > { fn SetCompleted (& self , handler : Ref < AsyncActionWithProgressCompletedHandler < P > >) -> Result < () > { self . 0 . set_completed (& self . as_interface () , handler) } fn Completed (& self) -> Result < AsyncActionWithProgressCompletedHandler < P > > { Err (Error :: empty ()) } fn GetResults (& self) -> Result < () > { self . 0 . get_results () } fn SetProgress (& self , _ : Ref < AsyncActionProgressHandler < P > >) -> Result < () > { Ok (()) } fn Progress (& self) -> Result < AsyncActionProgressHandler < P > > { Err (Error :: empty ()) } }
    };
}

impl_167!();
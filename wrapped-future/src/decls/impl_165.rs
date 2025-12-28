macro_rules! deps {
    () => {
        IAsyncAction_Impl!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl IAsyncAction_Impl for Action_Impl { fn SetCompleted (& self , handler : Ref < AsyncActionCompletedHandler >) -> Result < () > { self . 0 . set_completed (& self . as_interface () , handler) } fn Completed (& self) -> Result < AsyncActionCompletedHandler > { Err (Error :: empty ()) } fn GetResults (& self) -> Result < () > { self . 0 . get_results () } }
    };
}

impl_165!()
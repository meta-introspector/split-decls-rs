macro_rules! deps {
    () => {
        IAsyncAction_Impl!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl IAsyncAction_Impl for ReadyAction_Impl { fn SetCompleted (& self , handler : Ref < AsyncActionCompletedHandler >) -> Result < () > { self . 0 . invoke_completed (& self . as_interface () , handler) } fn Completed (& self) -> Result < AsyncActionCompletedHandler > { Err (Error :: empty ()) } fn GetResults (& self) -> Result < () > { self . 0 . result . clone () } }
    };
}

impl_143!()
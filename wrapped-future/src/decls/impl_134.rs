macro_rules! deps {
    () => {
        ReadyState!();
        AsyncStatus!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl < T : Async > ReadyState < T > { fn new (result : Result < T :: Output >) -> Self { Self { set_completed : AtomicBool :: new (false) , result , } } fn status (& self) -> AsyncStatus { if self . result . is_ok () { AsyncStatus :: Completed } else { AsyncStatus :: Error } } fn invoke_completed (& self , sender : & T , handler : Ref < T :: CompletedHandler >) -> Result < () > { if ! self . set_completed . swap (true , Ordering :: SeqCst) { sender . invoke_completed (handler . ok () ? , self . status ()) ; Ok (()) } else { Err (Error :: from_hresult (HRESULT (0x80000018u32 as i32))) } } fn error_code (& self) -> HRESULT { match & self . result { Ok (_) => HRESULT (0) , Err (error) => error . code () , } } }
    };
}

impl_134!()
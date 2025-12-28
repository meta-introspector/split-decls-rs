macro_rules! deps {
    () => {
        SyncState!();
        State!();
        AsyncStatus!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl < T : Async > SyncState < T > { fn new () -> Self { Self (Mutex :: new (State { result : None , completed : None , completed_assigned : false , })) } fn status (& self) -> AsyncStatus { self . 0 . lock () . unwrap () . status () } fn error_code (& self) -> HRESULT { self . 0 . lock () . unwrap () . error_code () } fn get_results (& self) -> Result < T :: Output > { self . 0 . lock () . unwrap () . get_results () } fn set_completed (& self , sender : & T , handler : Ref < T :: CompletedHandler >) -> Result < () > { let mut guard = self . 0 . lock () . unwrap () ; if guard . completed_assigned { Err (Error :: from_hresult (HRESULT (0x80000018u32 as i32))) } else { guard . completed_assigned = true ; let status = guard . status () ; let handler = handler . ok () ? ; if status == AsyncStatus :: Started { guard . completed = Some (handler . clone ()) ; } else { drop (guard) ; sender . invoke_completed (handler , status) ; } Ok (()) } } fn spawn < F > (& self , sender : & T , f : F) where F : FnOnce () -> Result < T :: Output > + Send + 'static , { let result = f () ; let mut guard = self . 0 . lock () . unwrap () ; debug_assert ! (guard . result . is_none ()) ; guard . result = Some (result) ; let status = guard . status () ; let completed = guard . completed . take () ; drop (guard) ; if let Some (completed) = completed { sender . invoke_completed (& completed , status) ; } } }
    };
}

impl_155!();
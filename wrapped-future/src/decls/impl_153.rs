macro_rules! deps {
    () => {
        State!();
        AsyncStatus!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl < T : Async > State < T > { fn status (& self) -> AsyncStatus { match & self . result { None => AsyncStatus :: Started , Some (Ok (_)) => AsyncStatus :: Completed , Some (Err (_)) => AsyncStatus :: Error , } } fn error_code (& self) -> HRESULT { match & self . result { Some (Err (error)) => error . code () , _ => HRESULT (0) , } } fn get_results (& self) -> Result < T :: Output > { match & self . result { Some (result) => result . clone () , None => Err (Error :: from_hresult (HRESULT (0x8000000Eu32 as i32))) , } } }
    };
}

impl_153!();
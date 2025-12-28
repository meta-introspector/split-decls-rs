macro_rules! deps {
    () => {
        Error!();
        ReadError!();
        Result!();
    };
}

macro_rules! impl_889 {
    () => {
        deps!();
        impl < T > ReadError < T > for Option < T > { fn read_error (self , error : & 'static str) -> Result < T > { self . ok_or (Error (error)) } }
    };
}

impl_889!()
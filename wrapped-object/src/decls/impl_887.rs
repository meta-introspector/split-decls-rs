macro_rules! deps {
    () => {
        ReadError!();
        Result!();
        Error!();
    };
}

macro_rules! impl_887 {
    () => {
        deps!();
        impl < T > ReadError < T > for result :: Result < T , () > { fn read_error (self , error : & 'static str) -> Result < T > { self . map_err (| () | Error (error)) } }
    };
}

impl_887!();
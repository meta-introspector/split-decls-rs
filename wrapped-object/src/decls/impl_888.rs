macro_rules! deps {
    () => {
        Result!();
        Error!();
        ReadError!();
    };
}

macro_rules! impl_888 {
    () => {
        deps!();
        impl < T > ReadError < T > for result :: Result < T , Error > { fn read_error (self , error : & 'static str) -> Result < T > { self . map_err (| _ | Error (error)) } }
    };
}

impl_888!()
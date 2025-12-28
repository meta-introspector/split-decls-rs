macro_rules! deps {
    () => {
        Error!();
        ReadError!();
        Result!();
    };
}

macro_rules! impl_742 {
    () => {
        deps!();
        impl < T > ReadError < T > for wasmparser :: Result < T > { fn read_error (self , error : & 'static str) -> Result < T > { self . map_err (| _ | Error (error)) } }
    };
}

impl_742!();
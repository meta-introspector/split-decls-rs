macro_rules! deps {
    () => {
        StringTypedError!();
        Error!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl From < String > for Error { fn from (message : String) -> Self { Box :: new (StringTypedError { message , source : None , }) . into () } }
    };
}

impl_1!()
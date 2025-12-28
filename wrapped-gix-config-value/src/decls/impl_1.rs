macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl Error { # [doc = " Create a new value error from `message`, with `input` being what's causing the error."] pub fn new (message : & 'static str , input : impl Into < bstr :: BString >) -> Self { Error { message , input : input . into () , utf8_err : None , } } pub (crate) fn with_err (mut self , err : std :: str :: Utf8Error) -> Self { self . utf8_err = Some (err) ; self } }
    };
}

impl_1!()
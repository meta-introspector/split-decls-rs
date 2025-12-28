macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! error {
    () => {
        deps!();
        mod error { use std :: fmt :: { Debug , Display , Formatter } ; use bstr :: BString ; # [doc = " The error representing an ERR packet line, as possibly wrapped into an `std::io::Error`."] # [derive (Debug)] pub struct Error { # [doc = " The contents of the ERR line, with `ERR` portion stripped."] pub message : BString , } impl Display for Error { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { Display :: fmt (& self . message , f) } } impl std :: error :: Error for Error { } }
    };
}

error!();
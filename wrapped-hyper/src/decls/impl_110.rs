macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl fmt :: Debug for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut f = f . debug_tuple ("hyper::Error") ; f . field (& self . inner . kind) ; if let Some (ref cause) = self . inner . cause { f . field (cause) ; } f . finish () } }
    };
}

impl_110!()
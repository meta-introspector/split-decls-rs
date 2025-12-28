macro_rules! deps {
    () => {
        Once!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl fmt :: Debug for Once { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Once") . field ("state" , & self . state ()) . finish () } }
    };
}

impl_35!();
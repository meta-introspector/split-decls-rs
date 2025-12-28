macro_rules! deps {
    () => {
        Connection!();
        Result!();
    };
}

macro_rules! impl_705 {
    () => {
        deps!();
        impl fmt :: Debug for Connection { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Connection") . field ("path" , & self . path ()) . finish () } }
    };
}

impl_705!();
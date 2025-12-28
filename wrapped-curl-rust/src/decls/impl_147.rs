macro_rules! deps {
    () => {
        SocketEvents!();
        Events!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl fmt :: Debug for SocketEvents { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Events") . field ("input" , & self . input ()) . field ("output" , & self . output ()) . field ("remove" , & self . remove ()) . finish () } }
    };
}

impl_147!()
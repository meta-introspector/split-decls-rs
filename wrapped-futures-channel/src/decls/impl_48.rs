macro_rules! deps {
    () => {
        TrySendError!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < T > fmt :: Debug for TrySendError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TrySendError") . field ("kind" , & self . err . kind) . finish () } }
    };
}

impl_48!()
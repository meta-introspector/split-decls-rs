macro_rules! deps {
    () => {
        UnboundedSender!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < T > fmt :: Debug for UnboundedSender < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("UnboundedSender") . field ("closed" , & self . is_closed ()) . finish () } }
    };
}

impl_76!();
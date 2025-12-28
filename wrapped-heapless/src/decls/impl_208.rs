macro_rules! deps {
    () => {
        Drain!();
        LenType!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl < LenT : LenType > fmt :: Debug for Drain < '_ , LenT > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Drain") . field (& self . as_str ()) . finish () } }
    };
}

impl_208!()
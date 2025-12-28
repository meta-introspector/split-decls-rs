macro_rules! deps {
    () => {
        LenType!();
        Drain!();
    };
}

macro_rules! impl_271 {
    () => {
        deps!();
        impl < T : fmt :: Debug , LenT : LenType > fmt :: Debug for Drain < '_ , T , LenT > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Drain") . field (& self . iter . as_slice ()) . finish () } }
    };
}

impl_271!()
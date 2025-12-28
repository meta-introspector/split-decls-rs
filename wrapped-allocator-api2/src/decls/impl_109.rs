macro_rules! deps {
    () => {
        Drain!();
        Allocator!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for Drain < '_ , T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Drain") . field (& self . iter . as_slice ()) . finish () } }
    };
}

impl_109!();
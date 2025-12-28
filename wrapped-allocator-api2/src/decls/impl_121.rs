macro_rules! deps {
    () => {
        Allocator!();
        IntoIter!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for IntoIter < T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("IntoIter") . field (& self . as_slice ()) . finish () } }
    };
}

impl_121!()
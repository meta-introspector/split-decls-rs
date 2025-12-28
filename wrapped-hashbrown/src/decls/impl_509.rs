macro_rules! deps {
    () => {
        IterHash!();
    };
}

macro_rules! impl_509 {
    () => {
        deps!();
        impl < T > fmt :: Debug for IterHash < '_ , T > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
    };
}

impl_509!();
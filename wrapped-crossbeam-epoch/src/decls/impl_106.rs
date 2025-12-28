macro_rules! deps {
    () => {
        Bag!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl fmt :: Debug for Bag { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Bag") . field ("deferreds" , & & self . deferreds [.. self . len]) . finish () } }
    };
}

impl_106!()
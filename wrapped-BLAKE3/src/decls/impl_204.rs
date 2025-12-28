macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! impl_204 {
    () => {
        deps!();
        impl fmt :: Debug for Hasher { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Hasher") . field ("flags" , & self . chunk_state . flags) . field ("platform" , & self . chunk_state . platform) . finish () } }
    };
}

impl_204!()
macro_rules! deps {
    () => {
        Result!();
        VersionFileId!();
    };
}

macro_rules! impl_1145 {
    () => {
        deps!();
        impl fmt :: Debug for VersionFileId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "VersionFileId({})" , self . 0) } }
    };
}

impl_1145!()
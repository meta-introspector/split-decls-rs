macro_rules! deps {
    () => {
        Result!();
        VersionId!();
    };
}

macro_rules! impl_1155 {
    () => {
        deps!();
        impl fmt :: Debug for VersionId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "VersionId({})" , self . 0) } }
    };
}

impl_1155!()
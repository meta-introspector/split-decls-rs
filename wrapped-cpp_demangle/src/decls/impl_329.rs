macro_rules! deps {
    () => {
        Result!();
        IndexStr!();
    };
}

macro_rules! impl_329 {
    () => {
        deps!();
        impl < 'a > fmt :: Debug for IndexStr < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "IndexStr {{ idx: {}, string: \"{}\" }}" , self . idx , String :: from_utf8_lossy (self . as_ref ())) } }
    };
}

impl_329!()
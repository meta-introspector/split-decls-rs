macro_rules! deps {
    () => {
        EntryMode!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl std :: fmt :: Debug for EntryMode { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "EntryMode(0o{})" , self . as_bytes (& mut Default :: default ())) } }
    };
}

impl_131!();
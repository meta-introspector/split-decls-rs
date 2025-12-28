macro_rules! deps {
    () => {
        EntryMode!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl std :: fmt :: Octal for EntryMode { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}" , self . as_bytes (& mut Default :: default ())) } }
    };
}

impl_132!()
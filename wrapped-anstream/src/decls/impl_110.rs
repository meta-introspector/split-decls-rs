macro_rules! deps {
    () => {
        Adapter!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < W > std :: fmt :: Write for Adapter < W > where W : FnMut (& [u8]) -> std :: io :: Result < () > , { fn write_str (& mut self , s : & str) -> std :: fmt :: Result { match (self . writer) (s . as_bytes ()) { Ok (()) => Ok (()) , Err (e) => { self . error = Err (e) ; Err (std :: fmt :: Error) } } } }
    };
}

impl_110!()
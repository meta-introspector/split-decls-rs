macro_rules! deps {
    () => {
        GUID!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl std :: fmt :: Display for GUID { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { write ! (f , "{:08x?}-{:04x?}-{:04x?}-{:02x?}{:02x?}-{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}" , self . 0 , self . 1 , self . 2 , self . 3 , self . 4 , self . 5 , self . 6 , self . 7 , self . 8 , self . 9 , self . 10) } }
    };
}

impl_34!();
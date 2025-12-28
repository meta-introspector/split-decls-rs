macro_rules! deps {
    () => {
        ObjectId!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl std :: fmt :: Debug for ObjectId { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { ObjectId :: Sha1 (_hash) => f . write_str ("Sha1(") ? , } for b in self . as_bytes () { write ! (f , "{b:02x}") ? ; } f . write_str (")") } }
    };
}

impl_14!();
macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl PartialOrd < String > for Bytes { fn partial_cmp (& self , other : & String) -> Option < cmp :: Ordering > { self . as_slice () . partial_cmp (other . as_bytes ()) } }
    };
}

impl_97!();
macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl PartialOrd < str > for Bytes { fn partial_cmp (& self , other : & str) -> Option < cmp :: Ordering > { self . as_slice () . partial_cmp (other . as_bytes ()) } }
    };
}

impl_89!()
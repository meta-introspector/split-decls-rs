macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl Ord for Bytes { fn cmp (& self , other : & Bytes) -> cmp :: Ordering { self . as_slice () . cmp (other . as_slice ()) } }
    };
}

impl_82!()
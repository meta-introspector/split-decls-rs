macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl PartialOrd for Bytes { fn partial_cmp (& self , other : & Bytes) -> Option < cmp :: Ordering > { Some (self . cmp (other)) } }
    };
}

impl_81!();
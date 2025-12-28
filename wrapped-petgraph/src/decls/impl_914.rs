macro_rules! deps {
    () => {
        Ptr!();
    };
}

macro_rules! impl_914 {
    () => {
        deps!();
        impl < 'b , T > PartialOrd for Ptr < 'b , T > { fn partial_cmp (& self , other : & Ptr < 'b , T >) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_914!()
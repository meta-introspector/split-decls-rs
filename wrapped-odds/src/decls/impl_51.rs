macro_rules! deps {
    () => {
        RevSlice!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < T , U > PartialEq < RevSlice < U > > for RevSlice < T > where T : PartialEq < U > , { fn eq (& self , rhs : & RevSlice < U >) -> bool { self . 0 == rhs . 0 } }
    };
}

impl_51!()
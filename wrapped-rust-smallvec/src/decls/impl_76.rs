macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < T , U , const N : usize > PartialEq < [U] > for SmallVec < T , N > where T : PartialEq < U > , { # [inline] fn eq (& self , other : & [U]) -> bool { self [..] == other [..] } }
    };
}

impl_76!()
macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < T , U , const N : usize > PartialEq < & mut [U] > for SmallVec < T , N > where T : PartialEq < U > , { # [inline] fn eq (& self , other : & & mut [U]) -> bool { self [..] == other [..] } }
    };
}

impl_78!()
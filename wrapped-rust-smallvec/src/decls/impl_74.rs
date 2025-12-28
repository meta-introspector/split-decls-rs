macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < T , U , const N : usize , const M : usize > PartialEq < [U ; M] > for SmallVec < T , N > where T : PartialEq < U > , { # [inline] fn eq (& self , other : & [U ; M]) -> bool { self [..] == other [..] } }
    };
}

impl_74!()
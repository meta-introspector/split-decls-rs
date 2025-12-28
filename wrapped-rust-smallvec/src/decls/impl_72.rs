macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < T , U , const N : usize , const M : usize > PartialEq < SmallVec < U , M > > for SmallVec < T , N > where T : PartialEq < U > , { # [inline] fn eq (& self , other : & SmallVec < U , M >) -> bool { self . as_slice () . eq (other . as_slice ()) } }
    };
}

impl_72!()
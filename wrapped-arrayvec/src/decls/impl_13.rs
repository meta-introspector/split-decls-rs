macro_rules! deps {
    () => {
        ArrayString!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < const CAP : usize > PartialEq < ArrayString < CAP > > for str { fn eq (& self , rhs : & ArrayString < CAP >) -> bool { self == & * * rhs } }
    };
}

impl_13!();
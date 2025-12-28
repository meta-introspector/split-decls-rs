macro_rules! deps {
    () => {
        ArrayString!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < const CAP : usize > PartialEq < str > for ArrayString < CAP > { fn eq (& self , rhs : & str) -> bool { & * * self == rhs } }
    };
}

impl_12!()
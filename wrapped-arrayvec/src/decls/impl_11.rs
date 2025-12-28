macro_rules! deps {
    () => {
        ArrayString!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < const CAP : usize > PartialEq for ArrayString < CAP > { fn eq (& self , rhs : & Self) -> bool { * * self == * * rhs } }
    };
}

impl_11!()
macro_rules! deps {
    () => {
        CppConst!();
    };
}

macro_rules! impl_240 {
    () => {
        deps!();
        impl Ord for CppConst { fn cmp (& self , other : & Self) -> Ordering { (self . field . name () , self) . cmp (& (other . field . name () , other)) } }
    };
}

impl_240!()
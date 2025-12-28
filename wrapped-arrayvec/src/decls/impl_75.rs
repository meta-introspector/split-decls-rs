macro_rules! deps {
    () => {
        ArrayVec!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < T , const CAP : usize > PartialEq < [T] > for ArrayVec < T , CAP > where T : PartialEq , { fn eq (& self , other : & [T]) -> bool { * * self == * other } }
    };
}

impl_75!();
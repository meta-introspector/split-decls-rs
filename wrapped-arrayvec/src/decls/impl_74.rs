macro_rules! deps {
    () => {
        ArrayVec!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < T , const CAP : usize > PartialEq for ArrayVec < T , CAP > where T : PartialEq , { fn eq (& self , other : & Self) -> bool { * * self == * * other } }
    };
}

impl_74!();
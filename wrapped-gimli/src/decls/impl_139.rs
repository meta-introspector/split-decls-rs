macro_rules! deps {
    () => {
        ArrayLike!();
        ArrayVec!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl < A : ArrayLike > PartialEq for ArrayVec < A > where A :: Item : PartialEq , { fn eq (& self , other : & Self) -> bool { * * self == * * other } }
    };
}

impl_139!();
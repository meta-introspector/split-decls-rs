macro_rules! deps {
    () => {
        ArrayLike!();
        ArrayVec!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl < A : ArrayLike > Clone for ArrayVec < A > where A :: Item : Clone , { fn clone (& self) -> Self { let mut new = Self :: default () ; for value in & * * self { new . try_push (value . clone ()) . unwrap () ; } new } }
    };
}

impl_138!()
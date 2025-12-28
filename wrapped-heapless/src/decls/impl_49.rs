macro_rules! deps {
    () => {
        Deque!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < T , const N : usize > Clone for Deque < T , N > where T : Clone , { fn clone (& self) -> Self { let mut res = Self :: new () ; for i in self { unsafe { res . push_back_unchecked (i . clone ()) } } res } }
    };
}

impl_49!()
macro_rules! deps {
    () => {
        Queue!();
    };
}

macro_rules! impl_459 {
    () => {
        deps!();
        impl < T , const N : usize > Clone for Queue < T , N > where T : Clone , { fn clone (& self) -> Self { let mut new : Self = Self :: new () ; for s in self . iter () { unsafe { new . enqueue_unchecked (s . clone ()) ; } } new } }
    };
}

impl_459!();
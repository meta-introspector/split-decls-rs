macro_rules! deps {
    () => {
        ArrayString!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < const CAP : usize > Clone for ArrayString < CAP > { fn clone (& self) -> ArrayString < CAP > { * self } fn clone_from (& mut self , rhs : & Self) { self . clear () ; self . try_push_str (rhs) . ok () ; } }
    };
}

impl_23!();
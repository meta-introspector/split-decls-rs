macro_rules! deps {
    () => {
        ArgFlags!();
        ArgSettings!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl ArgFlags { pub (crate) fn set (& mut self , setting : ArgSettings) { self . 0 |= setting . bit () ; } pub (crate) fn unset (& mut self , setting : ArgSettings) { self . 0 &= ! setting . bit () ; } pub (crate) fn is_set (& self , setting : ArgSettings) -> bool { self . 0 & setting . bit () != 0 } pub (crate) fn insert (& mut self , other : Self) { self . 0 |= other . 0 ; } }
    };
}

impl_70!()
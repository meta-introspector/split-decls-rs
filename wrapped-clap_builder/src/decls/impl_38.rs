macro_rules! deps {
    () => {
        AppFlags!();
        AppSettings!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl AppFlags { pub (crate) fn set (& mut self , setting : AppSettings) { self . 0 |= setting . bit () ; } pub (crate) fn unset (& mut self , setting : AppSettings) { self . 0 &= ! setting . bit () ; } pub (crate) fn is_set (& self , setting : AppSettings) -> bool { self . 0 & setting . bit () != 0 } pub (crate) fn insert (& mut self , other : Self) { self . 0 |= other . 0 ; } }
    };
}

impl_38!();
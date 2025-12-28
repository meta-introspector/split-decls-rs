macro_rules! deps {
    () => {
        Idx!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < T > PartialEq for Idx < T > { fn eq (& self , other : & Idx < T >) -> bool { self . raw == other . raw } }
    };
}

impl_31!();
macro_rules! deps {
    () => {
        Arc!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl < T : ? Sized + PartialEq > PartialEq for Arc < T > { fn eq (& self , other : & Arc < T >) -> bool { Self :: ptr_eq (self , other) || * (* self) == * (* other) } }
    };
}

impl_141!();
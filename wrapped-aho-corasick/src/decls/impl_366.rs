macro_rules! deps {
    () => {
        Pointer!();
    };
}

macro_rules! impl_366 {
    () => {
        deps!();
        impl < T > Pointer for * const T { fn as_usize (self) -> usize { self as usize } }
    };
}

impl_366!()
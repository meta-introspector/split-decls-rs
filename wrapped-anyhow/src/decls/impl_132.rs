macro_rules! deps {
    () => {
        Mut!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl < 'a , T > Mut < 'a , T > { pub unsafe fn read (self) -> T { unsafe { self . ptr . as_ptr () . read () } } }
    };
}

impl_132!();
macro_rules! deps {
    () => {
        Sample!();
        Distribution!();
    };
}

macro_rules! impl_380 {
    () => {
        deps!();
        impl < A > Deref for Distribution < A > { type Target = Sample < A > ; fn deref (& self) -> & Sample < A > { let slice : & [_] = & self . 0 ; unsafe { mem :: transmute (slice) } } }
    };
}

impl_380!()
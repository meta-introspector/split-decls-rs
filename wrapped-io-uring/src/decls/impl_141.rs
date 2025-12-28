macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl Clone for Entry { # [inline (always)] fn clone (& self) -> Entry { Entry (unsafe { mem :: transmute_copy (& self . 0) }) } }
    };
}

impl_141!();
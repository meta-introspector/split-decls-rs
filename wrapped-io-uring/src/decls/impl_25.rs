macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl Clone for Entry { fn clone (& self) -> Entry { Entry (unsafe { mem :: transmute_copy (& self . 0) }) } }
    };
}

impl_25!();
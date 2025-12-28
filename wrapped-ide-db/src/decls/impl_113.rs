macro_rules! deps {
    () => {
        IsEmpty!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl < T > IsEmpty for Vec < T > { fn is_empty (& self) -> bool { self . is_empty () } }
    };
}

impl_113!()
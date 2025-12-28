macro_rules! deps {
    () => {
        Tag!();
    };
}

macro_rules! impl_776 {
    () => {
        deps!();
        impl < 'repo > Clone for Tag < 'repo > { fn clone (& self) -> Self { self . as_object () . clone () . into_tag () . ok () . unwrap () } }
    };
}

impl_776!()
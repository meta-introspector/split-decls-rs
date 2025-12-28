macro_rules! deps {
    () => {
        Blob!();
    };
}

macro_rules! impl_219 {
    () => {
        deps!();
        impl < 'repo > Clone for Blob < 'repo > { fn clone (& self) -> Self { self . as_object () . clone () . into_blob () . ok () . unwrap () } }
    };
}

impl_219!()
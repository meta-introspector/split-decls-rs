macro_rules! deps {
    () => {
        Commit!();
    };
}

macro_rules! impl_261 {
    () => {
        deps!();
        impl < 'repo > Clone for Commit < 'repo > { fn clone (& self) -> Self { self . as_object () . clone () . into_commit () . ok () . unwrap () } }
    };
}

impl_261!()
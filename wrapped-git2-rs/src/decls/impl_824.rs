macro_rules! deps {
    () => {
        Tree!();
    };
}

macro_rules! impl_824 {
    () => {
        deps!();
        impl < 'repo > Clone for Tree < 'repo > { fn clone (& self) -> Self { self . as_object () . clone () . into_tree () . ok () . unwrap () } }
    };
}

impl_824!();
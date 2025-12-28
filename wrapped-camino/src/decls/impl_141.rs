macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl Ord for Utf8Path { fn cmp (& self , other : & Utf8Path) -> Ordering { self . components () . cmp (other . components ()) } }
    };
}

impl_141!()
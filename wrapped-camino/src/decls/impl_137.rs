macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl PartialEq for Utf8Path { # [inline] fn eq (& self , other : & Utf8Path) -> bool { self . components () . eq (other . components ()) } }
    };
}

impl_137!();
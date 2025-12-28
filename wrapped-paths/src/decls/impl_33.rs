macro_rules! deps {
    () => {
        RelPath!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl AsRef < Path > for RelPath { fn as_ref (& self) -> & Path { self . 0 . as_ref () } }
    };
}

impl_33!()
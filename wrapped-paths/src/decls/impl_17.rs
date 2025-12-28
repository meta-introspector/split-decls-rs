macro_rules! deps {
    () => {
        AbsPath!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl AsRef < Path > for AbsPath { fn as_ref (& self) -> & Path { self . 0 . as_ref () } }
    };
}

impl_17!();
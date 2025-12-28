macro_rules! deps {
    () => {
        AbsPath!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl AsRef < Utf8Path > for AbsPath { fn as_ref (& self) -> & Utf8Path { & self . 0 } }
    };
}

impl_16!()
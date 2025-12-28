macro_rules! deps {
    () => {
        RelPath!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl AsRef < Utf8Path > for RelPath { fn as_ref (& self) -> & Utf8Path { & self . 0 } }
    };
}

impl_32!();
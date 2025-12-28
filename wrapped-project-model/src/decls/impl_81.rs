macro_rules! deps {
    () => {
        ManifestPath!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl AsRef < Utf8Path > for ManifestPath { fn as_ref (& self) -> & Utf8Path { self . file . as_ref () } }
    };
}

impl_81!();
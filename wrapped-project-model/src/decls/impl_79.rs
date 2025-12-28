macro_rules! deps {
    () => {
        ManifestPath!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl AsRef < std :: path :: Path > for ManifestPath { fn as_ref (& self) -> & std :: path :: Path { self . file . as_ref () } }
    };
}

impl_79!();
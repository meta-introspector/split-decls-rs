macro_rules! deps {
    () => {
        ManifestPath!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl AsRef < AbsPath > for ManifestPath { fn as_ref (& self) -> & AbsPath { self . file . as_ref () } }
    };
}

impl_78!()
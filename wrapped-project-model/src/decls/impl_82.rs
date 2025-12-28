macro_rules! deps {
    () => {
        ManifestPath!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl Borrow < AbsPath > for ManifestPath { fn borrow (& self) -> & AbsPath { self . file . borrow () } }
    };
}

impl_82!();
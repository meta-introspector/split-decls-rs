macro_rules! deps {
    () => {
        ManifestPath!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl From < ManifestPath > for AbsPathBuf { fn from (it : ManifestPath) -> Self { it . file } }
    };
}

impl_74!();
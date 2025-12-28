macro_rules! deps {
    () => {
        ManifestPath!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl ManifestPath { pub fn parent (& self) -> & AbsPath { self . file . parent () . unwrap () } pub fn canonicalize (& self) -> ! { (* * self) . canonicalize () } pub fn is_rust_manifest (& self) -> bool { self . file . extension () == Some ("rs") } }
    };
}

impl_75!();
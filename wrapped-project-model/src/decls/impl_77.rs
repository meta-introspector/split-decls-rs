macro_rules! deps {
    () => {
        ManifestPath!();
        Target!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl ops :: Deref for ManifestPath { type Target = AbsPath ; fn deref (& self) -> & Self :: Target { & self . file } }
    };
}

impl_77!();
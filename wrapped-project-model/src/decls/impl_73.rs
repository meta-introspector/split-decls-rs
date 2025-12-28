macro_rules! deps {
    () => {
        ManifestPath!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl TryFrom < AbsPathBuf > for ManifestPath { type Error = AbsPathBuf ; fn try_from (file : AbsPathBuf) -> Result < Self , Self :: Error > { if file . parent () . is_none () { Err (file) } else { Ok (ManifestPath { file }) } } }
    };
}

impl_73!()
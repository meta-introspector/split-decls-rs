macro_rules! deps {
    () => {
        Manifest!();
    };
}

macro_rules! get_manifest {
    () => {
        deps!();
        pub (crate) fn get_manifest (manifest_dir : & Path) -> Manifest { try_get_manifest (manifest_dir) . unwrap_or_default () }
    };
}

get_manifest!()
macro_rules! deps {
    () => {
        PackageId!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        # [cfg (feature = "builder")] impl PackageBuilder { # [doc = " Construct a new `PackageBuilder` with all required fields."] pub fn new (name : impl Into < PackageName > , version : impl Into < Version > , id : impl Into < PackageId > , path : impl Into < Utf8PathBuf > ,) -> Self { Self :: default () . name (name) . version (version) . id (id) . manifest_path (path) } }
    };
}

impl_58!();
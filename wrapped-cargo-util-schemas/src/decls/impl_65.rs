macro_rules! deps {
    () => {
        TomlLockfilePackageIdError!();
        TomlLockfilePackageIdErrorKind!();
        TomlLockfileSourceIdError!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl From < TomlLockfileSourceIdError > for TomlLockfilePackageIdError { fn from (value : TomlLockfileSourceIdError) -> Self { TomlLockfilePackageIdErrorKind :: Source (value) . into () } }
    };
}

impl_65!();
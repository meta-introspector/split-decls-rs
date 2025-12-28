macro_rules! deps {
    () => {
        TomlLockfilePackageIdErrorKind!();
    };
}

macro_rules! TomlLockfilePackageIdError {
    () => {
        deps!();
        # [derive (Debug , thiserror :: Error)] # [error (transparent)] pub struct TomlLockfilePackageIdError (# [from] TomlLockfilePackageIdErrorKind) ;
    };
}

TomlLockfilePackageIdError!();
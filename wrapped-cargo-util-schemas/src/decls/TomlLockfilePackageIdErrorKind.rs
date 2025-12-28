macro_rules! deps {
    () => {
        TomlLockfileSourceIdError!();
    };
}

macro_rules! TomlLockfilePackageIdErrorKind {
    () => {
        deps!();
        # [non_exhaustive] # [derive (Debug , thiserror :: Error)] enum TomlLockfilePackageIdErrorKind { # [error ("invalid serialied PackageId")] InvalidSerializedPackageId , # [error (transparent)] Source (# [from] TomlLockfileSourceIdError) , }
    };
}

TomlLockfilePackageIdErrorKind!()
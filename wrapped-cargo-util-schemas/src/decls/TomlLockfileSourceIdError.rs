macro_rules! deps {
    () => {
        TomlLockfileSourceIdErrorKind!();
    };
}

macro_rules! TomlLockfileSourceIdError {
    () => {
        deps!();
        # [derive (Debug , thiserror :: Error)] # [error (transparent)] pub struct TomlLockfileSourceIdError (# [from] TomlLockfileSourceIdErrorKind) ;
    };
}

TomlLockfileSourceIdError!()
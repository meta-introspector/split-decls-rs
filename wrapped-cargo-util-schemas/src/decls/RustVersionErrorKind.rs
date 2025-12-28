macro_rules! deps {
    () => {
        PartialVersionError!();
        PartialVersion!();
        RustVersionError!();
    };
}

macro_rules! RustVersionErrorKind {
    () => {
        deps!();
        # [doc = " Non-public error kind for [`RustVersionError`]."] # [non_exhaustive] # [derive (Debug , thiserror :: Error)] enum RustVersionErrorKind { # [error ("unexpected prerelease field, expected a version like \"1.32\"")] Prerelease , # [error ("unexpected build field, expected a version like \"1.32\"")] BuildMetadata , # [error (transparent)] PartialVersion (# [from] PartialVersionError) , }
    };
}

RustVersionErrorKind!();
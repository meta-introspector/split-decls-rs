macro_rules! deps {
    () => {
        ErrorKind!();
    };
}

macro_rules! PackageIdSpecError {
    () => {
        deps!();
        # [derive (Debug , thiserror :: Error)] # [error (transparent)] pub struct PackageIdSpecError (# [from] ErrorKind) ;
    };
}

PackageIdSpecError!();
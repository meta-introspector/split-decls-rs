macro_rules! deps {
    () => {
        RustVersionErrorKind!();
        RustVersion!();
    };
}

macro_rules! RustVersionError {
    () => {
        deps!();
        # [doc = " Error parsing a [`RustVersion`]."] # [derive (Debug , thiserror :: Error)] # [error (transparent)] pub struct RustVersionError (# [from] RustVersionErrorKind) ;
    };
}

RustVersionError!();
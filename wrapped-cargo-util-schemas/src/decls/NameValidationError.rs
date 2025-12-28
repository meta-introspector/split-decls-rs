macro_rules! deps {
    () => {
        ErrorKind!();
    };
}

macro_rules! NameValidationError {
    () => {
        deps!();
        # [doc = " Error validating names in Cargo."] # [derive (Debug , thiserror :: Error)] # [error (transparent)] pub struct NameValidationError (# [from] ErrorKind) ;
    };
}

NameValidationError!();
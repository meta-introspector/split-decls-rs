macro_rules! deps {
    () => {
        PartialVersion!();
        ErrorKind!();
    };
}

macro_rules! PartialVersionError {
    () => {
        deps!();
        # [doc = " Error parsing a [`PartialVersion`]."] # [derive (Debug , thiserror :: Error)] # [error (transparent)] pub struct PartialVersionError (# [from] ErrorKind) ;
    };
}

PartialVersionError!()
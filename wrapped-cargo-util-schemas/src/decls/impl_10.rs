macro_rules! deps {
    () => {
        PartialVersionError!();
        ErrorKind!();
        PackageIdSpecError!();
        PartialVersion!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl From < PartialVersionError > for PackageIdSpecError { fn from (value : PartialVersionError) -> Self { ErrorKind :: PartialVersion (value) . into () } }
    };
}

impl_10!()
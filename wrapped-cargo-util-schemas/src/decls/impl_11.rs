macro_rules! deps {
    () => {
        NameValidationError!();
        PackageIdSpecError!();
        ErrorKind!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl From < NameValidationError > for PackageIdSpecError { fn from (value : NameValidationError) -> Self { ErrorKind :: NameValidation (value) . into () } }
    };
}

impl_11!()
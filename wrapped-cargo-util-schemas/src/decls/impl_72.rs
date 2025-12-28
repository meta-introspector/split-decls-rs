macro_rules! deps {
    () => {
        Result!();
        RustVersionErrorKind!();
        RustVersion!();
        PartialVersion!();
        RustVersionError!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl std :: str :: FromStr for RustVersion { type Err = RustVersionError ; fn from_str (value : & str) -> Result < Self , Self :: Err > { let partial = value . parse :: < PartialVersion > () ; let partial = partial . map_err (RustVersionErrorKind :: PartialVersion) ? ; partial . try_into () } }
    };
}

impl_72!();
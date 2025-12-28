macro_rules! deps {
    () => {
        PartialVersion!();
        RustVersion!();
        RustVersionErrorKind!();
        RustVersionError!();
        Result!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl TryFrom < PartialVersion > for RustVersion { type Error = RustVersionError ; fn try_from (partial : PartialVersion) -> Result < Self , Self :: Error > { if partial . pre . is_some () { return Err (RustVersionErrorKind :: Prerelease . into ()) ; } if partial . build . is_some () { return Err (RustVersionErrorKind :: BuildMetadata . into ()) ; } Ok (Self (partial)) } }
    };
}

impl_74!()
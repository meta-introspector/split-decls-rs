macro_rules! deps {
    () => {
        RustVersion!();
        RustVersionError!();
        PartialVersion!();
        Result!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl TryFrom < semver :: Version > for RustVersion { type Error = RustVersionError ; fn try_from (version : semver :: Version) -> Result < Self , Self :: Error > { let version = PartialVersion :: from (version) ; Self :: try_from (version) } }
    };
}

impl_73!()
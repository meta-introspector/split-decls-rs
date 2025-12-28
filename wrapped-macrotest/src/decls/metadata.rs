macro_rules! deps {
    () => {
        Metadata!();
        Result!();
        Error!();
    };
}

macro_rules! metadata {
    () => {
        deps!();
        pub (crate) fn metadata () -> Result < Metadata > { let output = raw_cargo () . arg ("metadata") . arg ("--format-version=1") . output () . map_err (Error :: Cargo) ? ; serde_json :: from_slice (& output . stdout) . map_err (Error :: CargoMetadata) }
    };
}

metadata!();
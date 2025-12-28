macro_rules! deps {
    () => {
        RustSourceWorkspaceConfig!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl RustSourceWorkspaceConfig { pub fn default_cargo () -> Self { RustSourceWorkspaceConfig :: CargoMetadata (Default :: default ()) } }
    };
}

impl_21!()
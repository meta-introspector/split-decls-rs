macro_rules! deps {
    () => {
        RustSourceWorkspaceConfig!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl Default for RustSourceWorkspaceConfig { fn default () -> Self { RustSourceWorkspaceConfig :: default_cargo () } }
    };
}

impl_147!()
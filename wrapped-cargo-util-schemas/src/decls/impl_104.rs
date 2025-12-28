macro_rules! deps {
    () => {
        WorkspaceValue!();
        TomlInheritedField!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl TomlInheritedField { pub fn new () -> Self { TomlInheritedField { workspace : WorkspaceValue , } } }
    };
}

impl_104!()
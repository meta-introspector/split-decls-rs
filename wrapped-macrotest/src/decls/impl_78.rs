macro_rules! deps {
    () => {
        WorkspacePackage!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl WorkspacePackage { fn is_none (& self) -> bool { self . edition . is_none () } }
    };
}

impl_78!()
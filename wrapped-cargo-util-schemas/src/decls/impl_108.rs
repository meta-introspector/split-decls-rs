macro_rules! deps {
    () => {
        WorkspaceValue!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl From < WorkspaceValue > for bool { fn from (_ : WorkspaceValue) -> bool { true } }
    };
}

impl_108!()
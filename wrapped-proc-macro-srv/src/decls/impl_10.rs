macro_rules! deps {
    () => {
        LoadProcMacroDylibError!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl From < io :: Error > for LoadProcMacroDylibError { fn from (e : io :: Error) -> Self { LoadProcMacroDylibError :: Io (e) } }
    };
}

impl_10!();
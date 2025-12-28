macro_rules! deps {
    () => {
        LoadProcMacroDylibError!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl From < libloading :: Error > for LoadProcMacroDylibError { fn from (e : libloading :: Error) -> Self { LoadProcMacroDylibError :: LibLoading (e) } }
    };
}

impl_11!();
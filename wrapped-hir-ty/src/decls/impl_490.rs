macro_rules! deps {
    () => {
        HirDisplayError!();
    };
}

macro_rules! impl_490 {
    () => {
        deps!();
        impl From < fmt :: Error > for HirDisplayError { fn from (_ : fmt :: Error) -> Self { Self :: FmtError } }
    };
}

impl_490!();
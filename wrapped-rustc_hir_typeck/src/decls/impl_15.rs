macro_rules! deps {
    () => {
        CastError!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl From < ErrorGuaranteed > for CastError < '_ > { fn from (err : ErrorGuaranteed) -> Self { CastError :: ErrorGuaranteed (err) } }
    };
}

impl_15!()
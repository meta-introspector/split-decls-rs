macro_rules! deps {
    () => {
        Error!();
        NTSTATUS!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl From < NTSTATUS > for Error { fn from (value : NTSTATUS) -> Self { value . to_hresult () . into () } }
    };
}

impl_108!()
macro_rules! deps {
    () => {
        Error!();
        RPC_STATUS!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl From < RPC_STATUS > for Error { fn from (value : RPC_STATUS) -> Self { value . to_hresult () . into () } }
    };
}

impl_115!();
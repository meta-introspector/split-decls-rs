macro_rules! deps {
    () => {
        HRESULT!();
        RPC_STATUS!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl From < RPC_STATUS > for HRESULT { fn from (value : RPC_STATUS) -> Self { value . to_hresult () } }
    };
}

impl_114!();
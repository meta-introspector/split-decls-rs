macro_rules! deps {
    () => {
        RPC_STATUS!();
        HRESULT!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl From < RPC_STATUS > for HRESULT { fn from (value : RPC_STATUS) -> Self { value . to_hresult () } }
    };
}

impl_114!()
macro_rules! deps {
    () => {
        NTSTATUS!();
        HRESULT!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl From < NTSTATUS > for HRESULT { fn from (value : NTSTATUS) -> Self { value . to_hresult () } }
    };
}

impl_107!();
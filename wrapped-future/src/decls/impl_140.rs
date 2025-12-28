macro_rules! deps {
    () => {
        IAsyncInfo_Impl!();
        AsyncStatus!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl < T : RuntimeType > IAsyncInfo_Impl for ReadyOperation_Impl < T > { fn Id (& self) -> Result < u32 > { Ok (1) } fn Status (& self) -> Result < AsyncStatus > { Ok (self . 0 . status ()) } fn ErrorCode (& self) -> Result < HRESULT > { Ok (self . 0 . error_code ()) } fn Cancel (& self) -> Result < () > { Ok (()) } fn Close (& self) -> Result < () > { Ok (()) } }
    };
}

impl_140!();
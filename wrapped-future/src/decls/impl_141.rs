macro_rules! deps {
    () => {
        IAsyncInfo_Impl!();
        AsyncStatus!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl < P : RuntimeType > IAsyncInfo_Impl for ReadyActionWithProgress_Impl < P > { fn Id (& self) -> Result < u32 > { Ok (1) } fn Status (& self) -> Result < AsyncStatus > { Ok (self . 0 . status ()) } fn ErrorCode (& self) -> Result < HRESULT > { Ok (self . 0 . error_code ()) } fn Cancel (& self) -> Result < () > { Ok (()) } fn Close (& self) -> Result < () > { Ok (()) } }
    };
}

impl_141!();
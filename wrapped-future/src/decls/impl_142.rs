macro_rules! deps {
    () => {
        AsyncStatus!();
        IAsyncInfo_Impl!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl < T : RuntimeType , P : RuntimeType > IAsyncInfo_Impl for ReadyOperationWithProgress_Impl < T , P > { fn Id (& self) -> Result < u32 > { Ok (1) } fn Status (& self) -> Result < AsyncStatus > { Ok (self . 0 . status ()) } fn ErrorCode (& self) -> Result < HRESULT > { Ok (self . 0 . error_code ()) } fn Cancel (& self) -> Result < () > { Ok (()) } fn Close (& self) -> Result < () > { Ok (()) } }
    };
}

impl_142!();
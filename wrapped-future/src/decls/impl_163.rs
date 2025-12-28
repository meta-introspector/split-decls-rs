macro_rules! deps {
    () => {
        AsyncStatus!();
        IAsyncInfo_Impl!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl < P : RuntimeType > IAsyncInfo_Impl for ActionWithProgress_Impl < P > { fn Id (& self) -> Result < u32 > { Ok (1) } fn Status (& self) -> Result < AsyncStatus > { Ok (self . 0 . status ()) } fn ErrorCode (& self) -> Result < HRESULT > { Ok (self . 0 . error_code ()) } fn Cancel (& self) -> Result < () > { Ok (()) } fn Close (& self) -> Result < () > { Ok (()) } }
    };
}

impl_163!()
macro_rules! deps {
    () => {
        AsyncStatus!();
        IAsyncInfo_Impl!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl IAsyncInfo_Impl for ReadyAction_Impl { fn Id (& self) -> Result < u32 > { Ok (1) } fn Status (& self) -> Result < AsyncStatus > { Ok (self . 0 . status ()) } fn ErrorCode (& self) -> Result < HRESULT > { Ok (self . 0 . error_code ()) } fn Cancel (& self) -> Result < () > { Ok (()) } fn Close (& self) -> Result < () > { Ok (()) } }
    };
}

impl_139!();
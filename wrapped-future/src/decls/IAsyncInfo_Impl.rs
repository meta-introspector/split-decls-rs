macro_rules! deps {
    () => {
        AsyncStatus!();
    };
}

macro_rules! IAsyncInfo_Impl {
    () => {
        deps!();
        pub trait IAsyncInfo_Impl : windows_core :: IUnknownImpl { fn Id (& self) -> windows_core :: Result < u32 > ; fn Status (& self) -> windows_core :: Result < AsyncStatus > ; fn ErrorCode (& self) -> windows_core :: Result < windows_core :: HRESULT > ; fn Cancel (& self) -> windows_core :: Result < () > ; fn Close (& self) -> windows_core :: Result < () > ; }
    };
}

IAsyncInfo_Impl!();
macro_rules! deps {
    () => {
        AsyncStatus!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl IAsyncInfo { pub fn Id (& self) -> windows_core :: Result < u32 > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Id) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . map (| | result__) } } pub fn Status (& self) -> windows_core :: Result < AsyncStatus > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . Status) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . map (| | result__) } } pub fn ErrorCode (& self) -> windows_core :: Result < windows_core :: HRESULT > { let this = self ; unsafe { let mut result__ = core :: mem :: zeroed () ; (windows_core :: Interface :: vtable (this) . ErrorCode) (windows_core :: Interface :: as_raw (this) , & mut result__ ,) . map (| | result__) } } pub fn Cancel (& self) -> windows_core :: Result < () > { let this = self ; unsafe { (windows_core :: Interface :: vtable (this) . Cancel) (windows_core :: Interface :: as_raw (this)) . ok () } } pub fn Close (& self) -> windows_core :: Result < () > { let this = self ; unsafe { (windows_core :: Interface :: vtable (this) . Close) (windows_core :: Interface :: as_raw (this)) . ok () } } }
    };
}

impl_73!();
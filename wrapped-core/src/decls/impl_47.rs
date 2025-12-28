macro_rules! deps {
    () => {
        Interface!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl IAgileReference { pub unsafe fn Resolve < T > (& self) -> windows_core :: Result < T > where T : windows_core :: Interface , { let mut result__ = core :: ptr :: null_mut () ; unsafe { (windows_core :: Interface :: vtable (self) . Resolve) (windows_core :: Interface :: as_raw (self) , & T :: IID , & mut result__ ,) . and_then (| | windows_core :: Type :: from_abi (result__)) } } }
    };
}

impl_47!()
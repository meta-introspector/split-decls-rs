macro_rules! deps {
    () => {
        ComPtr!();
        IUnknown_Vtbl!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl Clone for ComPtr { fn clone (& self) -> Self { unsafe { com_call ! (IUnknown_Vtbl , self . AddRef ()) ; } Self (self . 0) } }
    };
}

impl_46!()
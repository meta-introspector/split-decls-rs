macro_rules! deps {
    () => {
        IUnknown_Vtbl!();
        ComPtr!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl Drop for ComPtr { fn drop (& mut self) { unsafe { com_call ! (IUnknown_Vtbl , self . Release ()) ; } } }
    };
}

impl_47!();
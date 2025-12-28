macro_rules! deps {
    () => {
        ComPtr!();
        IUnknown_Vtbl!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl Drop for ComPtr { fn drop (& mut self) { unsafe { com_call ! (IUnknown_Vtbl , self . Release ()) ; } } }
    };
}

impl_47!()
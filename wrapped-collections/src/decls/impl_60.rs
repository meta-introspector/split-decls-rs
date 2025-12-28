macro_rules! deps {
    () => {
        IVector!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < T : windows_core :: RuntimeType + 'static > windows_core :: imp :: CanInto < windows_core :: IUnknown > for IVector < T > { }
    };
}

impl_60!()
macro_rules! deps {
    () => {
        IVector!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < T : windows_core :: RuntimeType + 'static > windows_core :: imp :: CanInto < windows_core :: IInspectable > for IVector < T > { }
    };
}

impl_61!();
macro_rules! deps {
    () => {
        IIterable!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl < T : windows_core :: RuntimeType + 'static > windows_core :: imp :: CanInto < windows_core :: IUnknown > for IIterable < T > { }
    };
}

impl_1!();
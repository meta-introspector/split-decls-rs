macro_rules! deps {
    () => {
        IIterable!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl < T : windows_core :: RuntimeType + 'static > windows_core :: imp :: CanInto < windows_core :: IInspectable > for IIterable < T > { }
    };
}

impl_2!();
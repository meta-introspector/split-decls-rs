macro_rules! deps {
    () => {
        IVectorView!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < T : windows_core :: RuntimeType + 'static > windows_core :: imp :: CanInto < windows_core :: IInspectable > for IVectorView < T > { }
    };
}

impl_74!()
macro_rules! deps {
    () => {
        IVectorView!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < T : windows_core :: RuntimeType + 'static > windows_core :: imp :: CanInto < windows_core :: IUnknown > for IVectorView < T > { }
    };
}

impl_73!();
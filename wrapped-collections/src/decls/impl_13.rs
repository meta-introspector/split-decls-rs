macro_rules! deps {
    () => {
        IIterator!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < T : windows_core :: RuntimeType + 'static > windows_core :: imp :: CanInto < windows_core :: IUnknown > for IIterator < T > { }
    };
}

impl_13!()
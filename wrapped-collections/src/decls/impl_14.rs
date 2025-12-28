macro_rules! deps {
    () => {
        IIterator!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < T : windows_core :: RuntimeType + 'static > windows_core :: imp :: CanInto < windows_core :: IInspectable > for IIterator < T > { }
    };
}

impl_14!()
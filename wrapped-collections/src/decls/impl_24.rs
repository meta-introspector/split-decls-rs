macro_rules! deps {
    () => {
        IKeyValuePair!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static > windows_core :: imp :: CanInto < windows_core :: IUnknown > for IKeyValuePair < K , V > { }
    };
}

impl_24!()
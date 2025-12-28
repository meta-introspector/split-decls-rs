macro_rules! deps {
    () => {
        IKeyValuePair!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static > windows_core :: imp :: CanInto < windows_core :: IInspectable > for IKeyValuePair < K , V > { }
    };
}

impl_25!();
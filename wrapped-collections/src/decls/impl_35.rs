macro_rules! deps {
    () => {
        IMap!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static > windows_core :: imp :: CanInto < windows_core :: IInspectable > for IMap < K , V > { }
    };
}

impl_35!()
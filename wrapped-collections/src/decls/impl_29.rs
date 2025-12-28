macro_rules! deps {
    () => {
        IKeyValuePair!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static > windows_core :: RuntimeName for IKeyValuePair < K , V > { const NAME : & 'static str = "Windows.Foundation.Collections.IKeyValuePair" ; }
    };
}

impl_29!()
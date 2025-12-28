macro_rules! deps {
    () => {
        IMap!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static > windows_core :: RuntimeName for IMap < K , V > { const NAME : & 'static str = "Windows.Foundation.Collections.IMap" ; }
    };
}

impl_42!()
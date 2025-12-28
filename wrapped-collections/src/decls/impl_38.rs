macro_rules! deps {
    () => {
        IIterable!();
        IMap!();
        IKeyValuePair!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < K : windows_core :: RuntimeType + 'static , V : windows_core :: RuntimeType + 'static > windows_core :: imp :: CanInto < IIterable < IKeyValuePair < K , V > > > for IMap < K , V > { const QUERY : bool = true ; }
    };
}

impl_38!();
macro_rules! deps {
    () => {
        IIterable!();
        IVector!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < T : windows_core :: RuntimeType + 'static > windows_core :: imp :: CanInto < IIterable < T > > for IVector < T > { const QUERY : bool = true ; }
    };
}

impl_64!()
macro_rules! deps {
    () => {
        IVectorView!();
        IIterable!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < T : windows_core :: RuntimeType + 'static > windows_core :: imp :: CanInto < IIterable < T > > for IVectorView < T > { const QUERY : bool = true ; }
    };
}

impl_77!()
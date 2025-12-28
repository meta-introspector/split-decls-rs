macro_rules! deps {
    () => {
        IIterable!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < T : windows_core :: RuntimeType + 'static > windows_core :: RuntimeName for IIterable < T > { const NAME : & 'static str = "Windows.Foundation.Collections.IIterable" ; }
    };
}

impl_6!()
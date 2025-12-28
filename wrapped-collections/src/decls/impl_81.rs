macro_rules! deps {
    () => {
        IVectorView!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < T : windows_core :: RuntimeType + 'static > windows_core :: RuntimeName for IVectorView < T > { const NAME : & 'static str = "Windows.Foundation.Collections.IVectorView" ; }
    };
}

impl_81!()
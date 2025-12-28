macro_rules! deps {
    () => {
        IVector!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < T : windows_core :: RuntimeType + 'static > windows_core :: RuntimeName for IVector < T > { const NAME : & 'static str = "Windows.Foundation.Collections.IVector" ; }
    };
}

impl_68!()
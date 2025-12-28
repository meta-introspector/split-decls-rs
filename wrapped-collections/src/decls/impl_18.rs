macro_rules! deps {
    () => {
        IIterator!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < T : windows_core :: RuntimeType + 'static > windows_core :: RuntimeName for IIterator < T > { const NAME : & 'static str = "Windows.Foundation.Collections.IIterator" ; }
    };
}

impl_18!();
macro_rules! deps {
    () => {
        RuntimeName!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl windows_core :: RuntimeName for IWeakReferenceSource { }
    };
}

impl_65!()
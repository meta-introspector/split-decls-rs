macro_rules! deps {
    () => {
        RuntimeName!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl windows_core :: RuntimeName for IWeakReference { }
    };
}

impl_58!()
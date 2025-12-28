macro_rules! deps {
    () => {
        RuntimeName!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl windows_core :: RuntimeName for IAgileObject { }
    };
}

impl_44!();
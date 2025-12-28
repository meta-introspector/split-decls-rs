macro_rules! deps {
    () => {
        IUnknownImpl!();
    };
}

macro_rules! IAgileObject_Impl {
    () => {
        deps!();
        pub trait IAgileObject_Impl : windows_core :: IUnknownImpl { }
    };
}

IAgileObject_Impl!();
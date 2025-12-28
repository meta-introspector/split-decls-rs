macro_rules! deps {
    () => {
        IIterator!();
    };
}

macro_rules! IIterable_Impl {
    () => {
        deps!();
        pub trait IIterable_Impl < T > : windows_core :: IUnknownImpl where T : windows_core :: RuntimeType + 'static , { fn First (& self) -> windows_core :: Result < IIterator < T > > ; }
    };
}

IIterable_Impl!();
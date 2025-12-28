macro_rules! deps {
    () => {
        IUnknownImpl!();
    };
}

macro_rules! IWeakReferenceSource_Impl {
    () => {
        deps!();
        pub trait IWeakReferenceSource_Impl : windows_core :: IUnknownImpl { fn GetWeakReference (& self) -> windows_core :: Result < IWeakReference > ; }
    };
}

IWeakReferenceSource_Impl!()
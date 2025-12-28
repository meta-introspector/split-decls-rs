macro_rules! deps {
    () => {
        IUnknown!();
    };
}

macro_rules! macro_60 {
    () => {
        deps!();
        windows_core :: imp :: interface_hierarchy ! (IWeakReferenceSource , windows_core :: IUnknown) ;
    };
}

macro_60!()
macro_rules! deps {
    () => {
        IUnknown!();
    };
}

macro_rules! macro_46 {
    () => {
        deps!();
        windows_core :: imp :: interface_hierarchy ! (IAgileReference , windows_core :: IUnknown) ;
    };
}

macro_46!();
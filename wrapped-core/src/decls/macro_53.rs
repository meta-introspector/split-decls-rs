macro_rules! deps {
    () => {
        IUnknown!();
    };
}

macro_rules! macro_53 {
    () => {
        deps!();
        windows_core :: imp :: interface_hierarchy ! (IWeakReference , windows_core :: IUnknown) ;
    };
}

macro_53!();
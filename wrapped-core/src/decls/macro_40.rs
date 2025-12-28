macro_rules! deps {
    () => {
        IUnknown!();
    };
}

macro_rules! macro_40 {
    () => {
        deps!();
        windows_core :: imp :: interface_hierarchy ! (IAgileObject , windows_core :: IUnknown) ;
    };
}

macro_40!();
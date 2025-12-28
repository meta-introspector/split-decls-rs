macro_rules! deps {
    () => {
        IInspectable!();
        IUnknown!();
    };
}

macro_rules! macro_146 {
    () => {
        deps!();
        interface_hierarchy ! (IInspectable , IUnknown) ;
    };
}

macro_146!();
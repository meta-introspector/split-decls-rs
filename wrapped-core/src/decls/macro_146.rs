macro_rules! deps {
    () => {
        IUnknown!();
        IInspectable!();
    };
}

macro_rules! macro_146 {
    () => {
        deps!();
        interface_hierarchy ! (IInspectable , IUnknown) ;
    };
}

macro_146!()
macro_rules! deps {
    () => {
        OnInformationalCallback!();
        OnInformational!();
    };
}

macro_rules! on_informational_raw {
    () => {
        deps!();
        pub (crate) fn on_informational_raw < B , C > (req : & mut http :: Request < B > , callback : C) where C : OnInformationalCallback + Send + Sync + 'static , { req . extensions_mut () . insert (OnInformational (Arc :: new (callback))) ; }
    };
}

on_informational_raw!();
macro_rules! deps {
    () => {
        DefaultAutoderefCtx!();
        AutoderefKind!();
        GeneralAutoderef!();
    };
}

macro_rules! Autoderef {
    () => {
        deps!();
        pub (crate) type Autoderef < 'a , 'db , Steps = Vec < (Ty < 'db > , AutoderefKind) > > = GeneralAutoderef < 'db , DefaultAutoderefCtx < 'a , 'db > , Steps > ;
    };
}

Autoderef!();
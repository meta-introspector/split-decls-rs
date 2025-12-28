macro_rules! deps {
    () => {
        GeneralAutoderef!();
        AutoderefKind!();
        DefaultAutoderefCtx!();
    };
}

macro_rules! Autoderef {
    () => {
        deps!();
        pub (crate) type Autoderef < 'a , 'db , Steps = Vec < (Ty < 'db > , AutoderefKind) > > = GeneralAutoderef < 'db , DefaultAutoderefCtx < 'a , 'db > , Steps > ;
    };
}

Autoderef!()
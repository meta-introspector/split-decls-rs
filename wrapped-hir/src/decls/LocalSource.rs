macro_rules! deps {
    () => {
        SelfParam!();
        Local!();
    };
}

macro_rules! LocalSource {
    () => {
        deps!();
        pub struct LocalSource { pub local : Local , pub source : InFile < Either < ast :: IdentPat , ast :: SelfParam > > , }
    };
}

LocalSource!()
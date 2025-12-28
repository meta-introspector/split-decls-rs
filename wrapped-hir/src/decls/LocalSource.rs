macro_rules! deps {
    () => {
        Local!();
        SelfParam!();
    };
}

macro_rules! LocalSource {
    () => {
        deps!();
        pub struct LocalSource { pub local : Local , pub source : InFile < Either < ast :: IdentPat , ast :: SelfParam > > , }
    };
}

LocalSource!()